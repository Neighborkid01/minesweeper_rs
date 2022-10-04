use cell::Cell;
use yew::*;
use gloo_console as console;
use gloo::timers::callback::Interval;
use rand::Rng;
use yew::html::Scope;
use std::collections::HashSet;

mod cell;

enum Msg {
    Tick,
    MouseDown,
    Click(usize),
    RightClick(usize),
    // Chord(usize),
    Reset,
}

enum Face {
    Happy,
    Nervous,
    Dead,
    Cool,
}

impl Face {
    fn as_str(&self) -> &'static str {
        match self {
            Face::Happy     => "🙂",
            Face::Nervous   => "😬",
            Face::Dead      => "😵",
            Face::Cool      => "😎",
        }
    }
}

struct App {
    active:                 bool,
    face:                   Face,
    cells:                  Vec<Cell>,
    mines:                  Vec<Cell>,
    cells_width:            usize,
    cells_height:           usize,
    shown_cells_count:      usize,
    seconds_played:         usize,
    interval:               Option<Interval>,
}

impl App {
    fn reset(&mut self) {
        let (cells, _) = generate_cells(self.cells_width, self.cells_height, self.mines.len());
        self.reassign_cells(&cells);
        self.seconds_played = 0;
    }

    fn reassign_cells(&mut self, cells: &Vec<Cell>) {
        for (index, cell) in cells.iter().enumerate() {
            self.cells[index] = *cell;
        }
    }

    fn click_neighboring_empty_cells(&mut self, index: usize) {
        let (row, col) = get_row_col_from_index(index, self.cells_width, self.cells_height);
        let neighbors = neighbors(row, col, self.cells_width, self.cells_height);
        for index in neighbors.iter() {
            console::log!("clicking index:", *index);
            self.handle_click(*index, None);
        }
    }

    fn count_flagged_mines(&self) -> usize {
        self.cells.iter().filter(|cell| cell.is_flagged()).count()
    }

    fn view_cell(&self, index: usize, cell: &Cell, link: &Scope<Self>) -> Html {
        let is_shown = {
            if cell.is_shown() { "clicked" } else { "" }
        };
        let is_mine = {
            if cell.is_mine() { "mine" } else { "" }
        };
        let value = cell.get_value_string();

        html! {
            <td key={index}
                class={classes!("cell", is_shown, is_mine)}
                onclick={link.callback(move |_| Msg::Click(index))}
            >
                {value}
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
        self.active = true;
        self.face = Face::Happy;
        self.reset();
        console::log!("Resetting...");
        true
    }

    fn handle_click(&mut self, index: usize, _ctx: Option<&Context<Self>>) -> bool {
        if !self.active { return false; }

        if self.interval.is_none() {
            let callback = _ctx.unwrap().link().callback(|_| Msg::Tick);
            let interval = Interval::new(1000, move || callback.emit(()));
            self.interval = Some(interval);
        }

        let mut cell = self.cells[index];
        if cell.is_shown() { return false; }
        cell.handle_click();
        self.cells[index] = cell; // Need to reassign cell or its changes aren't saved

        if cell.is_mine() {
            self.active = false;
            self.face = Face::Dead;
            // self.interval.as_ref().unwrap().cancel();
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

    fn handle_right_click(&self, index: usize) -> bool {
        let mut cell = self.cells[index];
        cell.cycle_display();
        true
    }

    fn check_for_win(&mut self) {
        if self.shown_cells_count + self.mines.len() == self.cells.len() {
            self.handle_win();
        }
    }

    fn handle_win(&mut self) {
        self.active = false;
        self.face = Face::Cool;
        // self.interval.unwrap().cancel();
        self.interval = None;
        self.flag_all_mines();
    }

    fn flag_all_mines(&self) {
        for i in 0..self.mines.len() {
            let mut mine = self.mines[i];
            mine.set_display_to_flagged();
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

        let (cells, mines) = generate_cells(cells_width, cells_height, mines_count);

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
                console::log!("ticking");
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
                    .map(|(x, cell)| self.view_cell(index_offset + x, cell, ctx.link()));
                html! {
                    <tr key={y} class="game-row">
                        { for row_cells }
                    </tr>
                }
            });

        html! {
            <>
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

                // <script type="text/javascript">
                //     // Disabling right click
                //     document.oncontextmenu = function(event) { event.preventDefault(); };

                //     window.game = new Game();
                //     selectChanged()

                //     function selectChanged() {
                //         var difficulty = document.getElementById("difficulty").value;
                //         game.setup(difficulty);
                //     }
                // </script>
            </>
        }
    }
}

fn generate_cells(cells_width: usize, cells_height: usize, mines_count: usize) -> (Vec<Cell>, Vec<Cell>) {
    let mut cells: Vec<Cell> = Vec::new();
    let mut mines: Vec<Cell> = Vec::new();
    let mut mine_indicies: HashSet<usize> = HashSet::new();
    for _ in 0..mines_count {
        let mut i = get_random_cell_index(cells_width, cells_height);

        while mine_indicies.contains(&i) {
            i = get_random_cell_index(cells_width, cells_height);
        }
        mine_indicies.insert(i);
        console::log!(i);
    }

    let cells_count = cells_width * cells_height;
    for index in 0..cells_count {
        let (row, col) = get_row_col_from_index(index, cells_width, cells_height);
        let neighbors = neighbors(row, col, cells_width, cells_height);
        let value = Cell::calculate_value(index, &neighbors, &mine_indicies);
        let cell = Cell::new(value);

        if mine_indicies.contains(&index) { mines.push(cell); }
        cells.push(cell);
    }

    (cells, mines)
}

fn get_random_cell_index(cells_width: usize, cells_height: usize) -> usize {
    let mut rng = rand::thread_rng();
    rng.gen_range(0..(cells_width * cells_height))
}

fn neighbors(row: usize, col: usize, cells_width: usize, cells_height: usize) -> HashSet<usize> {
    let mut neighbors: HashSet<usize> = HashSet::new();

    if let Some(n) = get_index_from_row_col(row - 1, col - 1, cells_width, cells_height) { neighbors.insert(n); }
    if let Some(n) = get_index_from_row_col(row - 1, col, cells_width, cells_height) { neighbors.insert(n); }
    if let Some(n) = get_index_from_row_col(row - 1, col + 1, cells_width, cells_height) { neighbors.insert(n); }
    if let Some(n) = get_index_from_row_col(row, col - 1, cells_width, cells_height) { neighbors.insert(n); }
    if let Some(n) = get_index_from_row_col(row, col + 1, cells_width, cells_height) { neighbors.insert(n); }
    if let Some(n) = get_index_from_row_col(row + 1, col - 1, cells_width, cells_height) { neighbors.insert(n); }
    if let Some(n) = get_index_from_row_col(row + 1, col, cells_width, cells_height) { neighbors.insert(n); }
    if let Some(n) = get_index_from_row_col(row + 1, col + 1, cells_width, cells_height) { neighbors.insert(n); }

    neighbors
}

fn get_index_from_row_col(row: usize, col: usize, cells_width: usize, cells_height: usize) -> Option<usize> {
    if row < cells_height && col < cells_width {
        Some(row * cells_width + col)
    } else {
        None
    }
}

fn get_row_col_from_index(index: usize, cells_width: usize, cells_height: usize) -> (usize, usize) {
    let row = index / cells_height;
    let col = index % cells_width;

    (row as usize, col as usize)
}

fn main() {
    yew::start_app::<App>();
}