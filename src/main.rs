mod components;
mod models;

use components::counter::Counter;
use models::face::Face;
use models::cell::Cell as Cell;
use models::mouse_state::MouseState;
use models::settings::{Difficulty, Settings, Dimensions};
use wasm_bindgen::JsCast;
use yew::{html, Component, Context, Html, classes};
use web_sys::{Element, MouseEvent};
// use gloo_console as console;
use gloo::timers::callback::Interval;
use rand::Rng;
use std::{collections::HashSet, cmp};

enum Msg {
    Tick,
    MouseDown(usize, MouseEvent),
    MouseUp(usize, MouseEvent),
    MouseMove(MouseEvent),
    Reset,
    Ignore,
    ForceRender,
    ChangeSize(Difficulty),
}

struct App {
    active:                     bool,
    face:                       Face,
    cells:                      Vec<Cell>,
    neighbors:                  Vec<HashSet<usize>>,
    mine_indices:               Vec<usize>,
    shown_cells_count:          usize,
    selected_cell_index:        Option<usize>,
    first_clicked_mine_index:   Option<usize>,
    seconds_played:             usize,
    mouse_state:                MouseState,
    settings:                   Settings,
    interval:                   Option<Interval>,
}

impl App {
    fn reset_interval(&mut self, ctx: &Context<Self>) {
        let callback = ctx.link().callback(|_| Msg::Tick);
        let interval = Interval::new(1000, move || callback.emit(()));
        self.interval = Some(interval);
    }

    fn reassign_cells(&mut self, index_clicked: usize) {
        let (cells, neighbors, mine_indices) = self.generate_cells(index_clicked);
        self.cells = cells;
        self.neighbors = neighbors;
        self.mine_indices = mine_indices;
    }

    fn clear_cells(&mut self) {
        for cell in self.cells.iter_mut() {
            cell.reset();
        }
        for mine_index in self.mine_indices.iter_mut() {
            // Setting to 0 because it will be updated in reassign_cells
            *mine_index = 0;
        }
    }

    fn generate_cells(&self, index_clicked: usize) -> (Vec<Cell>, Vec<HashSet<usize>>, Vec<usize>) {
        let mut cells: Vec<Cell> = Vec::new();
        let mut neighbors: Vec<HashSet<usize>> = Vec::new();
        let mut mine_indices: Vec<usize> = Vec::new();
        let mut current_mine_indices: HashSet<usize> = HashSet::new();
        let index_neighbors = self.calculate_neighbors(index_clicked);
        for _ in 0..self.mine_indices.len() {
            let mut i = self.get_random_cell_index();
            while !self.index_can_be_mine(index_clicked, i, &current_mine_indices, &index_neighbors) {
                i = self.get_random_cell_index();
            }
            current_mine_indices.insert(i);
        }

        for cell_index in 0..self.cells.len() {
            let neighboring_cells = self.calculate_neighbors(cell_index);
            let neighboring_mines = if current_mine_indices.contains(&cell_index) {
                None
            } else {
                Some(neighboring_cells.intersection(&current_mine_indices).count())
            };
            let cell = Cell::new(neighboring_mines);
            cells.push(cell);
            neighbors.push(neighboring_cells);
            if current_mine_indices.contains(&cell_index) { mine_indices.push(cell_index); }
        }

        (cells, neighbors, mine_indices)
    }

    fn index_can_be_mine(&self, index_clicked: usize, mine_index: usize, current_mine_indices: &HashSet<usize>, neighbors: &HashSet<usize>) -> bool {
        if current_mine_indices.contains(&mine_index) { return false; }
        if index_clicked == mine_index { return self.settings.first_click_setting_is_any(); }
        if neighbors.contains(&mine_index) && self.settings.first_click_setting_is_zero() { return false; }
        true
    }


    fn calculate_neighbors(&self, index: usize) -> HashSet<usize> {
        let (row, col) = self.get_row_col_from_index(index);
        let mut neighbors: HashSet<usize> = HashSet::new();
        let r = row as isize;
        let c = col as isize;

        if let Some(n) = self.get_index_from_row_col(r - 1, c - 1)  { neighbors.insert(n); }
        if let Some(n) = self.get_index_from_row_col(r - 1, c)      { neighbors.insert(n); }
        if let Some(n) = self.get_index_from_row_col(r - 1, c + 1)  { neighbors.insert(n); }
        if let Some(n) = self.get_index_from_row_col(r, c - 1)      { neighbors.insert(n); }
        if let Some(n) = self.get_index_from_row_col(r, c + 1)      { neighbors.insert(n); }
        if let Some(n) = self.get_index_from_row_col(r + 1, c - 1)  { neighbors.insert(n); }
        if let Some(n) = self.get_index_from_row_col(r + 1, c)      { neighbors.insert(n); }
        if let Some(n) = self.get_index_from_row_col(r + 1, c + 1)  { neighbors.insert(n); }

        neighbors
    }

    fn get_random_cell_index(&self) -> usize {
        let mut rng = rand::thread_rng();
        rng.gen_range(0..(self.settings.dimensions().width() * self.settings.dimensions().height()))
    }

    fn get_index_from_row_col(&self, row: isize, col: isize) -> Option<usize> {
        if row >= 0 && (row as usize) < self.settings.dimensions().height() &&
           col >= 0 && (col as usize) < self.settings.dimensions().width() {
            Some((row as usize * self.settings.dimensions().width()) + col as usize)
        } else {
            None
        }
    }

    fn get_row_col_from_index(&self, index: usize) -> (usize, usize) {
        let row = index / self.settings.dimensions().width();
        let col = index % self.settings.dimensions().width();

        (row, col)
    }

    fn neighbors_selected_cell(&self, index: usize) -> bool {
        let Some(selected_index) = self.selected_cell_index else { return false; };
        if index == selected_index { return true; }

        let neigbors = &self.neighbors[selected_index];
        neigbors.contains(&index)
    }

    fn click_neighboring_empty_cells(&mut self, index: usize) {
        let neighbors = self.neighbors[index].clone();
        for index in neighbors {
            self.handle_click(index, None);
        }
    }

    fn click_all_mines(&mut self, ctx: Option<&Context<Self>>) {
        for i in 0..self.mine_indices.len() {
            let index = self.mine_indices[i];
            self.handle_click(index, ctx);
        }
    }

    fn count_flagged_mines(&self) -> usize {
        self.cells.iter().filter(|cell| cell.is_flagged()).count()
    }

    fn no_game_in_proggress(&self) -> bool {
        self.interval.is_none()
    }

    fn view_cell(&self, index: usize, cell: &Cell, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        let value = cell.get_value_display_string();
        let cell_is_shown = cell.is_shown();
        let cell_at_selected_index_is_shown = self.selected_cell_index.is_some() && self.cells[self.selected_cell_index.unwrap()].is_shown();
        let cell_is_at_selected_index = self.selected_cell_index.is_some() && index == self.selected_cell_index.unwrap();
        let cell_is_first_clicked_mine = self.first_clicked_mine_index.is_some() && index == self.first_clicked_mine_index.unwrap();
        let state_is_chording = self.mouse_state.is_chording(self.settings.chord_setting(), cell_at_selected_index_is_shown) && self.neighbors_selected_cell(index);

        let mine  = { if cell_is_shown && cell.is_mine() && (cell_is_at_selected_index || cell_is_first_clicked_mine) { "mine" } else { "" } };
        let shown = { if !cell.is_flagged() && (cell_is_shown || cell_is_at_selected_index || state_is_chording) { "clicked" } else { "" } };

        let onmousedown = link.callback(move |e: MouseEvent| Msg::MouseDown(index, e));
        let onmouseup   = link.callback(move |e: MouseEvent| Msg::MouseUp(index, e));

        html! {
            <td key={index}
                class={classes!("cell-border")} {onmousedown} {onmouseup}
            >
                <div class={classes!("cell", shown, mine, cell.color().to_string())}>{value}</div>
            </td>
        }
    }

    fn check_difficulty_is_eq(&self, difficulty: Difficulty) -> bool {
        // https://stackoverflow.com/questions/32554285/compare-enums-only-by-variant-not-value
        std::mem::discriminant(&self.settings.difficulty()) == std::mem::discriminant(&difficulty)
    }

    fn handle_change_size(&mut self, difficulty: Difficulty) -> bool {
        self.settings.set_difficulty(difficulty);
        self.cells = vec![Cell::new_empty(); self.settings.dimensions().width() * self.settings.dimensions().height()];
        self.neighbors = vec![HashSet::new(); self.settings.dimensions().width() * self.settings.dimensions().height()];
        self.mine_indices = vec![0; self.settings.dimensions().mines()];
        self.handle_reset();
        true
    }

    fn handle_mouse_down(&mut self, index: usize, event: MouseEvent) -> bool {
        if !self.active { return false; }
        self.mouse_state = self.mouse_state.mouse_down(event);
        self.face = Face::Nervous;

        match self.mouse_state {
            MouseState::Left | MouseState::Both => {
                self.selected_cell_index = Some(index);
                true
            },
            MouseState::Right => { self.handle_right_click(index) },
            MouseState::AfterBoth | MouseState::Neither => { false }
        }
    }

    fn handle_mouse_up(&mut self, index: usize, event: MouseEvent, ctx: Option<&Context<Self>>) -> bool {
        if !self.active { return  false; }
        let new_mouse_state = self.mouse_state.mouse_up(event);
        match self.mouse_state {
            MouseState::AfterBoth | MouseState::Neither => {
                self.mouse_state = new_mouse_state;
                true
            },
            MouseState::Left => {
                if !new_mouse_state.is_neither() { return true; }
                let Some(selected_cell_index) = self.selected_cell_index else { return false; };
                if selected_cell_index != index { return true; }

                let chord_setting = self.settings.chord_setting();
                let cell_is_shown = self.cells[index].is_shown();
                let is_chording = self.mouse_state.is_chording(chord_setting, cell_is_shown);

                self.mouse_state = new_mouse_state;
                if is_chording {
                    self.handle_chord(index, ctx);
                    true
                } else {
                    self.handle_click(index, ctx)
                }
            },
            MouseState::Right => {
                self.mouse_state = new_mouse_state;
                false
            },
            MouseState::Both => {
                self.handle_chord(index, ctx);
                self.mouse_state = new_mouse_state;
                true
            }
        }
    }

    fn handle_mouse_move(&mut self, event: MouseEvent) -> bool {
        if self.selected_cell_index.is_none() || self.mouse_state.is_neither() { return false; }
        let rect = event
            .target()
            .expect("mouse event doesn't have a target")
            .dyn_into::<Element>()
            .expect("event target should be of type HtmlElement")
            .get_bounding_client_rect();
        let x = (event.client_x() as f64) - rect.left();
        let y = (event.client_y() as f64) - rect.top();

        if x <= 0.0 || y <= 0.0 {
            self.selected_cell_index = None;
            self.mouse_state = MouseState::Neither;
            self.face = Face::Happy;
            return true;
        }

        false
    }

    fn handle_tick(&mut self) -> bool {
        self.seconds_played += 1;
        if self.seconds_played > 999 { self.seconds_played = 999; }
        true
    }

    fn handle_reset(&mut self) -> bool {
        self.interval = None;
        self.face = Face::Happy;
        self.seconds_played = 0;
        self.shown_cells_count = 0;
        self.clear_cells();
        self.first_clicked_mine_index = None;
        self.active = true;
        true
    }

    fn handle_click(&mut self, index: usize, ctx: Option<&Context<Self>>) -> bool {
        if !self.active { return false; }

        if self.no_game_in_proggress() {
            self.reassign_cells(index);
            self.reset_interval(ctx.unwrap());
        }

        if self.cells[index].is_shown() || self.cells[index].is_flagged() {
            self.face = Face::Happy;
            return true;
        }

        self.cells[index].handle_click();

        if self.cells[index].is_mine() {
            let Some(selected_index) = self.selected_cell_index else { return false; };
            if self.first_clicked_mine_index.is_none() && (index == selected_index || self.neighbors[selected_index].contains(&index)) {
                self.first_clicked_mine_index = Some(index);
                self.click_all_mines(ctx);
                self.active = false;
                self.face = Face::Dead;
                self.interval = None;
            }
            return true;
        }

        self.face = Face::Happy;
        self.shown_cells_count += 1;

        // Recursively click all neighboring cells if we clicked a 0
        if self.cells[index].is_zero() { self.click_neighboring_empty_cells(index); }
        self.check_for_win();
        true
    }

    fn handle_right_click(&mut self, index: usize) -> bool {
        if !self.active { return false; }

        self.cells[index].cycle_display(self.settings.allow_mark_cell_as_unknown());
        self.face = Face::Happy;
        true
    }

    fn handle_chord(&mut self, index: usize, ctx: Option<&Context<Self>>) -> bool {
        if !self.cells[index].is_shown() { return false; }

        let neighbors = self.neighbors[index].clone();
        let neighboring_mines = neighbors.iter().filter(|index| self.cells[**index].is_mine()).count();
        let neighboring_flags = neighbors.iter().filter(|index| self.cells[**index].is_flagged()).count();
        if neighboring_mines != neighboring_flags { return false; }

        for index in neighbors {
            self.handle_click(index, ctx);
        }
        true
    }

    fn check_for_win(&mut self) {
        if self.shown_cells_count + self.mine_indices.len() == self.cells.len() {
            self.handle_win();
        }
    }

    fn handle_win(&mut self) {
        self.active = false;
        self.face = Face::Cool;
        self.flag_all_mines();
        self.interval = None;
    }

    fn flag_all_mines(&mut self) {
        for index in &self.mine_indices {
            self.cells[*index].set_display_to_flagged();
        }
    }

}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let settings = Settings::default();
        let shown_cells_count   = 0;
        let seconds_played      = 0;

        let cells = vec![Cell::new_empty(); settings.dimensions().width() * settings.dimensions().height()];
        let neighbors = vec![HashSet::new(); settings.dimensions().width() * settings.dimensions().height()];
        let mines = vec![0; settings.dimensions().mines()];

        Self {
            active: true,
            face: Face::Happy,
            cells,
            neighbors,
            mine_indices: mines,
            shown_cells_count,
            seconds_played,
            selected_cell_index: None,
            first_clicked_mine_index: None,
            mouse_state: MouseState::Neither,
            settings,
            interval: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Msg) -> bool {
        match msg {
            Msg::ChangeSize(difficulty) => {
                self.handle_change_size(difficulty)
            },
            Msg::MouseDown(index, event) => {
                self.handle_mouse_down(index, event)
            },
            Msg::MouseUp(index, event) => {
                self.handle_mouse_up(index, event, Some(ctx))
            },
            Msg::MouseMove(event) => {
                self.handle_mouse_move(event)
            }
            Msg::Tick => {
                self.handle_tick()
            },
            Msg::Reset => {
                self.handle_reset()
            },
            Msg::Ignore => { false },
            Msg::ForceRender => { true },
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let flagged_mines_count = self.count_flagged_mines() as isize;
        let mines_remaining = cmp::max(self.mine_indices.len() as isize - flagged_mines_count, -99);

        let highlight_beginner = if self.check_difficulty_is_eq(Difficulty::Beginner) { "highlight" } else { "" };
        let highlight_intermediate = if self.check_difficulty_is_eq(Difficulty::Intermediate) { "highlight" } else { "" };
        let highlight_expert = if self.check_difficulty_is_eq(Difficulty::Expert) { "highlight" } else { "" };
        let highlight_custom = if self.check_difficulty_is_eq(Difficulty::Custom(Dimensions::new(0, 0, 0))) { "highlight" } else { "" }; // The specific dimensions don't matter here

        let cell_rows = self.cells
            .chunks(self.settings.dimensions().width())
            .enumerate()
            .map(|(y, cells)| {
                let index_offset = y * self.settings.dimensions().width();

                let row_cells = cells
                    .iter()
                    .enumerate()
                    .map(|(x, cell)| self.view_cell(index_offset + x, cell, ctx));
                html! {
                    <tr key={y} class="game-row">
                        { for row_cells }
                    </tr>
                }
            });

        html! {
            <div class="container no-select">
                <div class="settings">
                    <a class={classes!("difficulty", highlight_beginner)}
                       onclick={ctx.link().callback(move |_| Msg::ChangeSize(Difficulty::Beginner))}
                    >
                        { "Beginner" }
                    </a>
                    <a class={classes!("difficulty", highlight_intermediate)}
                       onclick={ctx.link().callback(move |_| Msg::ChangeSize(Difficulty::Intermediate))}
                    >
                        { "Intermediate" }
                    </a>
                    <a class={classes!("difficulty", highlight_expert)}
                       onclick={ctx.link().callback(move |_| Msg::ChangeSize(Difficulty::Expert))}
                    >
                        { "Expert" }
                    </a>
                    <a class={classes!("difficulty", highlight_custom)}
                       onclick={ctx.link().callback(move |_| Msg::ChangeSize(Difficulty::Custom(Dimensions::new(32, 32, 250))))}
                    >
                        { "Custom" }
                    </a>
                </div>

                <div class="header">
                    <Counter value={mines_remaining} classes="left" />
                    <div id="resetButtonContainer" class="center">
                        <span id="resetButton" onclick={ctx.link().callback(move |_| Msg::Reset)}>{ self.face.as_str() }</span>
                    </div>
                    <Counter value={self.seconds_played as isize} classes="right" />
                </div>

                <div class="board-container">
                    <table id="board" class="board"
                        oncontextmenu={ ctx.link().callback(move |e: MouseEvent| { e.prevent_default(); Msg::Ignore }) }
                        onmousemove={ ctx.link().callback(Msg::MouseMove)}
                    >
                        { for cell_rows }
                    </table>
                </div>
            </div>
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, _: bool) {
        if self.selected_cell_index.is_some() && self.mouse_state.is_neither() && self.active {
            self.selected_cell_index = None;
            ctx.link().callback(move |_| {Msg::ForceRender}).emit(());
        }
    }
}

fn main() {
    yew::start_app::<App>();
}