use cell::Cell;
use face::Face;
use yew::{html, Component, Context, Html, classes};
use web_sys::MouseEvent;
use gloo_console as console;
use gloo::timers::callback::Interval;
use rand::Rng;
use std::collections::HashSet;

mod cell;
mod face;

enum Msg {
    Tick,
    MouseDown,
    Click(usize),
    RightClick(usize),
    // Chord(usize),
    Reset,
}

struct App {
    active:                 bool,
    face:                   Face,
    cells:                  Vec<Cell>,
    mines:                  Vec<usize>,
    cells_width:            usize,
    cells_height:           usize,
    shown_cells_count:      usize,
    seconds_played:         usize,
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
        self.face = Face::Happy;
        self.seconds_played = 0;
        self.shown_cells_count = 0;
        self.active = true;
    }

    fn reassign_cells(&mut self, index: usize) {
        let (cells, mines) = self.generate_cells(index);
        for (index, cell) in cells.iter().enumerate() {
            self.cells[index] = *cell;
        }
        for (index, mine) in mines.iter().enumerate() {
            self.mines[index] = *mine;
        }
    }

    fn generate_cells(&self, index: usize) -> (Vec<Cell>, Vec<usize>) {
        let mut cells: Vec<Cell> = Vec::new();
        let mut mines: Vec<usize> = Vec::new();
        let mut mine_indicies: HashSet<usize> = HashSet::new();
        for _ in 0..self.mines.len() {
            let mut i = self.get_random_cell_index();
            while index == i || mine_indicies.contains(&i) {
                i = self.get_random_cell_index();
            }
            mine_indicies.insert(i);
        }

        let cells_count = self.cells_width * self.cells_height;
        for index in 0..cells_count {
            let (row, col) = self.get_row_col_from_index(index);
            let neighbors = self.neighbors(row, col);
            let value = Cell::calculate_value(index, &neighbors, &mine_indicies);
            let cell = Cell::new(value);

            if mine_indicies.contains(&index) { mines.push(index); }
            cells.push(cell);
        }

        (cells, mines)
    }


    fn neighbors(&self, row: usize, col: usize) -> HashSet<usize> {
        let mut neighbors: HashSet<usize> = HashSet::new();

        if let Some(n) = self.get_index_from_row_col(row - 1, col - 1) { neighbors.insert(n); }
        if let Some(n) = self.get_index_from_row_col(row - 1, col) { neighbors.insert(n); }
        if let Some(n) = self.get_index_from_row_col(row - 1, col + 1) { neighbors.insert(n); }
        if let Some(n) = self.get_index_from_row_col(row, col - 1) { neighbors.insert(n); }
        if let Some(n) = self.get_index_from_row_col(row, col + 1) { neighbors.insert(n); }
        if let Some(n) = self.get_index_from_row_col(row + 1, col - 1) { neighbors.insert(n); }
        if let Some(n) = self.get_index_from_row_col(row + 1, col) { neighbors.insert(n); }
        if let Some(n) = self.get_index_from_row_col(row + 1, col + 1) { neighbors.insert(n); }

        neighbors
    }

    fn get_random_cell_index(&self) -> usize {
        let mut rng = rand::thread_rng();
        rng.gen_range(0..(self.cells_width * self.cells_height))
    }

    fn get_index_from_row_col(&self, row: usize, col: usize) -> Option<usize> {
        if row < self.cells_height && col < self.cells_width {
            Some(row * self.cells_width + col)
        } else {
            None
        }
    }

    fn get_row_col_from_index(&self, index: usize) -> (usize, usize) {
        let row = index / self.cells_height;
        let col = index % self.cells_width;

        (row as usize, col as usize)
    }

    fn click_neighboring_empty_cells(&mut self, index: usize) {
        let (row, col) = self.get_row_col_from_index(index);
        let neighbors = self.neighbors(row, col);
        for index in neighbors.iter() {
            self.handle_click(*index, None);
        }
    }

    fn count_flagged_mines(&self) -> usize {
        self.cells.iter().filter(|cell| cell.is_flagged()).count()
    }

    fn view_cell(&self, index: usize, cell: &Cell, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        let value = cell.get_value_display_string();
        let is_shown = cell.is_shown();
        let shown = {
            if is_shown { "clicked" } else { "" }
        };
        let mine = {
            if is_shown && cell.is_mine() { "mine" } else { "" }
        };
        let color = {
            // This has to be a String instead of &str because the enum lifetime and cell's lifetime are different or something
            if is_shown { cell.value.get_name_string() } else { String::from("") }
        };

        html! {
            <td key={ index }
                class={ classes!("cell", shown, mine, color) }
                onclick={ link.callback(move |_| Msg::Click(index)) }
                oncontextmenu={ link.callback(move |e: MouseEvent| {
                    e.prevent_default();
                    Msg::RightClick(index)
                }) }
            >
                { value }
            </td>
        }
    }

    fn handle_mouse_down(&mut self) -> bool {
        if self.active { self.face = Face::Nervous; }
        true
    }

    fn handle_tick(&mut self) -> bool {
        self.seconds_played += 1;
        true
    }

    fn handle_reset(&mut self) -> bool {
        self.reset();
        true
    }

    fn handle_click(&mut self, index: usize, ctx: Option<&Context<Self>>) -> bool {
        if !self.active { return false; }

        if self.interval.is_none() {
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
        let cells_width         = 10;
        let cells_height        = 10;
        let mines_count         = 10;
        let shown_cells_count   = 0;
        let seconds_played      = 0;

        let cells = vec![Cell::new_empty(); cells_width * cells_height];
        let mines = vec![0; mines_count];

        console::log!("Done");
        Self {
            active: true,
            face: Face::Happy,
            cells,
            mines,
            cells_width,
            cells_height,
            shown_cells_count,
            seconds_played,
            interval: None,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Msg) -> bool {
        match msg {
            Msg::MouseDown => {
                self.handle_mouse_down()
            }
            Msg::Tick => {
                self.handle_tick()
            },
            Msg::Reset => {
                self.handle_reset()
            },
            Msg::Click(index) => {
                self.handle_click(index, Some(_ctx))
            },
            Msg::RightClick(index) => {
                self.handle_right_click(index)
            },
            // Msg::Chord(_) => todo!(),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let flagged_mines_count = self.count_flagged_mines() as isize;
        let mines_remaining = self.mines.len() as isize - flagged_mines_count;

        let cell_rows = self.cells
            .chunks(self.cells_width)
            .enumerate()
            .map(|(y, cells)| {
                let index_offset = y * self.cells_width;

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
                <div class="header">
                    <div id="minesRemainingContainer" class="counter left">
                        <span id="minesRemaining">{ mines_remaining }</span>
                    </div>
                    <div id="resetButtonContainer" class="center">
                        <span id="resetButton" onclick={ctx.link().callback(move |_| Msg::Reset)}>{ self.face.as_str() }</span>
                    </div>
                    <div id="timerContainer" class="counter right">
                        <span id="timer">{ self.seconds_played }</span>
                    </div>
                </div>

                <table id="board" class="board" onmousedown={ctx.link().callback(move |_| Msg::MouseDown)}>
                    { for cell_rows }
                </table>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<App>();
}