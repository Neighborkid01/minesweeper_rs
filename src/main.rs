use cell::Cell;
use face::Face;
use mouse_state::MouseState;
use settings::{Difficulty, Settings, Dimensions, ChordSetting};
use wasm_bindgen::JsCast;
use yew::{html, Component, Context, Html, classes};
use web_sys::{Element, MouseEvent};
use gloo_console as console;
use gloo::timers::callback::Interval;
use rand::Rng;
use std::collections::HashSet;

mod cell;
mod face;
mod mouse_state;
mod settings;

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
    active:                 bool,
    face:                   Face,
    cells:                  Vec<Cell>,
    neighbors:              Vec<HashSet<usize>>,
    mines:                  Vec<usize>,
    shown_cells_count:      usize,
    selected_cell_index:    Option<usize>,
    seconds_played:         usize,
    mouse_state:            MouseState,
    first_click_is_zero:    bool,
    settings:               Settings,
    interval:               Option<Interval>,
}

impl App {
    fn reset_interval(&mut self, ctx: &Context<Self>) {
        let callback = ctx.link().callback(|_| Msg::Tick);
        let interval = Interval::new(1000, move || callback.emit(()));
        self.interval = Some(interval);
    }

    fn reset(&mut self) {
        console::log!("Resetting...");
        self.interval = None;
        self.face = Face::Happy;
        self.seconds_played = 0;
        self.shown_cells_count = 0;
        self.clear_cells();
        self.active = true;
    }

    fn reassign_cells(&mut self, index: usize) {
        let (cells, neighbors, mines) = self.generate_cells(index);
        for (index, cell) in cells.iter().enumerate() {
            self.cells[index] = *cell;
        }
        for (index, neighbor) in neighbors.iter().enumerate() {
            self.neighbors[index] = neighbor.clone();
        }
        for (index, mine) in mines.iter().enumerate() {
            self.mines[index] = *mine;
        }
    }

    fn clear_cells(&mut self) {
        for index in 0..self.cells.len() {
            let mut cell = self.cells[index];
            cell.reset();
            self.cells[index] = cell;
        }
        for index in 0..self.mines.len() {
            self.mines[index] = 0;
        }
    }

    fn generate_cells(&self, index: usize) -> (Vec<Cell>, Vec<HashSet<usize>>, Vec<usize>) {
        let mut cells: Vec<Cell> = Vec::new();
        let mut neighbors: Vec<HashSet<usize>> = Vec::new();
        let mut mines: Vec<usize> = Vec::new();
        let mut mine_indicies: HashSet<usize> = HashSet::new();
        let index_neighbors = self.calculate_neighbors(index);
        for _ in 0..self.mines.len() {
            let mut i = self.get_random_cell_index();
            while self.index_check(index, i, &mine_indicies, &index_neighbors) {
                i = self.get_random_cell_index();
            }
            mine_indicies.insert(i);
        }

        for cell_index in 0..self.cells.len() {
            let neighboring_cells = self.calculate_neighbors(cell_index);
            let neighboring_mines = if mine_indicies.contains(&cell_index) {
                None
            } else {
                Some(neighboring_cells.intersection(&mine_indicies).count())
            };
            let cell = Cell::new(neighboring_mines);
            cells.push(cell);
            neighbors.push(neighboring_cells);
            if mine_indicies.contains(&cell_index) { mines.push(cell_index); }
        }

        (cells, neighbors, mines)
    }

    fn index_check(&self, index: usize, mine_index: usize, mine_indicies: &HashSet<usize>, neighbors: &HashSet<usize>) -> bool {
        let index_check = index == mine_index || mine_indicies.contains(&mine_index);
        if !self.first_click_is_zero { return index_check; }

        index_check || neighbors.contains(&mine_index)
    }


    fn calculate_neighbors(&self, index: usize) -> HashSet<usize> {
        let (row, col) = self.get_row_col_from_index(index);
        let mut neighbors: HashSet<usize> = HashSet::new();
        let r = row as isize;
        let c = col as isize;

        if let Some(n) = self.get_index_from_row_col(r - 1, c - 1) { neighbors.insert(n); }
        if let Some(n) = self.get_index_from_row_col(r - 1, c) { neighbors.insert(n); }
        if let Some(n) = self.get_index_from_row_col(r - 1, c + 1) { neighbors.insert(n); }
        if let Some(n) = self.get_index_from_row_col(r, c - 1) { neighbors.insert(n); }
        if let Some(n) = self.get_index_from_row_col(r, c + 1) { neighbors.insert(n); }
        if let Some(n) = self.get_index_from_row_col(r + 1, c - 1) { neighbors.insert(n); }
        if let Some(n) = self.get_index_from_row_col(r + 1, c) { neighbors.insert(n); }
        if let Some(n) = self.get_index_from_row_col(r + 1, c + 1) { neighbors.insert(n); }

        neighbors
    }

    fn get_random_cell_index(&self) -> usize {
        let mut rng = rand::thread_rng();
        rng.gen_range(0..(self.settings.dimensions.width * self.settings.dimensions.height))
    }

    fn get_index_from_row_col(&self, row: isize, col: isize) -> Option<usize> {
        if row >= 0 && (row as usize) < self.settings.dimensions.height &&
           col >= 0 && (col as usize) < self.settings.dimensions.width {
            Some((row as usize * self.settings.dimensions.width) + col as usize)
        } else {
            None
        }
    }

    fn get_row_col_from_index(&self, index: usize) -> (usize, usize) {
        let row = index / self.settings.dimensions.width;
        let col = index % self.settings.dimensions.width;

        (row as usize, col as usize)
    }

    fn neighbors_selected_cell(&self, index: usize) -> bool {
        if self.selected_cell_index.is_none() { return false; }

        let selected_index = self.selected_cell_index.unwrap();
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
        for i in 0..self.mines.len() {
            let index = self.mines[i];
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
        let state_is_chording = self.mouse_state.is_chording(self.settings.chord_setting, cell_at_selected_index_is_shown) && self.neighbors_selected_cell(index);

        // This has to be a String instead of &str because the enum lifetime and cell's lifetime are different or something
        let color = { if cell_is_shown { cell.value.get_name_string() } else { String::from("") } };
        let mine  = { if cell_is_shown && cell.is_mine() && cell_is_at_selected_index { "mine" } else { "" } };
        let shown = { if !cell.is_flagged() && (cell_is_shown || cell_is_at_selected_index || state_is_chording) { "clicked" } else { "" } };

        let onmousedown = link.callback(move |e: MouseEvent| Msg::MouseDown(index, e));
        let onmouseup   = link.callback(move |e: MouseEvent| Msg::MouseUp(index, e));

        html! {
            <td key={index}
                class={classes!("cell-border")} {onmousedown} {onmouseup}
            >
                <div class={classes!("cell", shown, mine, color)}>{value}</div>
            </td>
        }
    }

    fn check_difficulty_is_eq(&self, difficulty: Difficulty) -> bool {
        // https://stackoverflow.com/questions/32554285/compare-enums-only-by-variant-not-value
        std::mem::discriminant(&self.settings.difficulty) == std::mem::discriminant(&difficulty)
    }

    fn handle_change_size(&mut self, difficulty: Difficulty, chord_setting: ChordSetting) -> bool {
        self.settings = Settings::new(difficulty, chord_setting);
        self.cells = vec![Cell::new_empty(); self.settings.dimensions.width * self.settings.dimensions.height];
        self.neighbors = vec![HashSet::new(); self.settings.dimensions.width * self.settings.dimensions.height];
        self.mines = vec![0; self.settings.dimensions.mines];
        self.reset();
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
                if self.selected_cell_index.is_none() { return false; }
                if self.selected_cell_index.unwrap() != index { return true; }

                let chord_setting = self.settings.chord_setting;
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
            return true;
        }

        false
    }

    fn handle_tick(&mut self) -> bool {
        self.seconds_played += 1;
        if self.seconds_played >= 999 { self.interval = None; }
        true
    }

    fn handle_reset(&mut self) -> bool {
        self.reset();
        true
    }

    fn handle_click(&mut self, index: usize, ctx: Option<&Context<Self>>) -> bool {
        if !self.active { return false; }

        if self.no_game_in_proggress() {
            self.reassign_cells(index);
            self.reset_interval(ctx.unwrap());
        }

        let mut cell = self.cells[index];
        if cell.is_shown() || cell.is_flagged() {
            self.face = Face::Happy;
            return true;
        }

        cell.handle_click();
        self.cells[index] = cell; // Need to reassign cell or its changes aren't saved

        if cell.is_mine() {
            if self.selected_cell_index.is_none() || index != self.selected_cell_index.unwrap() {
                return false;
            } else {
                self.click_all_mines(ctx);
            }
            self.active = false;
            self.face = Face::Dead;
            self.interval = None;
        } else {
            self.face = Face::Happy;
            self.shown_cells_count += 1;
        }

        // Recursively click all neighboring cells if we clicked a 0
        if cell.is_zero() { self.click_neighboring_empty_cells(index); }

        self.check_for_win();
        true
    }

    fn handle_right_click(&mut self, index: usize) -> bool {
        if !self.active { return false; }

        let mut cell = self.cells[index];
        cell.cycle_display();
        self.cells[index] = cell;
        self.face = Face::Happy;
        true
    }

    fn handle_chord(&mut self, index: usize, ctx: Option<&Context<Self>>) -> bool {
        let cell = self.cells[index];
        if !cell.is_shown() { return false; }

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
        if self.shown_cells_count + self.mines.len() == self.cells.len() {
            self.handle_win();
        }
    }

    fn handle_win(&mut self) {
        console::log!("you won!");
        self.active = false;
        self.face = Face::Cool;
        self.flag_all_mines();
        self.interval = None;
    }

    fn flag_all_mines(&mut self) {
        for index in &self.mines {
            let mut mine = self.cells[*index];
            mine.set_display_to_flagged();
            self.cells[*index] = mine;
        }
    }

}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        console::log!("Building app...");
        let difficulty = Difficulty::Beginner;
        let chord_setting = ChordSetting::LeftClick;
        let settings = Settings::new(difficulty, chord_setting);
        let shown_cells_count   = 0;
        let seconds_played      = 0;

        let cells = vec![Cell::new_empty(); settings.dimensions.width * settings.dimensions.height];
        let neighbors = vec![HashSet::new(); settings.dimensions.width * settings.dimensions.height];
        let mines = vec![0; settings.dimensions.mines];

        console::log!("Done");
        Self {
            active: true,
            face: Face::Happy,
            cells,
            neighbors,
            mines,
            shown_cells_count,
            seconds_played,
            selected_cell_index: None,
            mouse_state: MouseState::Neither,
            first_click_is_zero: true,
            settings,
            interval: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Msg) -> bool {
        match msg {
            Msg::ChangeSize(difficulty) => {
                self.handle_change_size(difficulty, self.settings.chord_setting)
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
        let mines_remaining = self.mines.len() as isize - flagged_mines_count;

        let highlight_beginner = if self.check_difficulty_is_eq(Difficulty::Beginner) { "highlight" } else { "" };
        let highlight_intermediate = if self.check_difficulty_is_eq(Difficulty::Intermediate) { "highlight" } else { "" };
        let highlight_expert = if self.check_difficulty_is_eq(Difficulty::Expert) { "highlight" } else { "" };
        let highlight_custom = if self.check_difficulty_is_eq(Difficulty::Custom(Dimensions { width: 0, height: 0, mines: 0 })) { "highlight" } else { "" }; // The specific dimensions don't matter here

        let cell_rows = self.cells
            .chunks(self.settings.dimensions.width)
            .enumerate()
            .map(|(y, cells)| {
                let index_offset = y * self.settings.dimensions.width;

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
                    <div id="minesRemainingContainer" class="counter left">
                        <span id="minesRemaining">{ format!("{:0>3}", mines_remaining)  }</span>
                    </div>
                    <div id="resetButtonContainer" class="center">
                        <span id="resetButton" onclick={ctx.link().callback(move |_| Msg::Reset)}>{ self.face.as_str() }</span>
                    </div>
                    <div id="timerContainer" class="counter right">
                        <span id="timer">{ format!("{:0>3}", self.seconds_played) }</span>
                    </div>
                </div>

                <div class="board-container">
                    <table id="board" class="board"
                        oncontextmenu={ ctx.link().callback(move |e: MouseEvent| { e.prevent_default(); Msg::Ignore }) }
                        onmousemove={ ctx.link().callback(move |e: MouseEvent| Msg::MouseMove(e))}
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