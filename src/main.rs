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
            Face::Happy => "🙂",
            Face::Nervous => "😬",
            Face::Dead => "😵",
            Face::Cool => "😎",
        }
    }
}

struct App {
    active:         bool,
    face:           Face,
    cells:          Vec<Cell>,
    cells_width:    usize,
    cells_height:   usize,
    mines_count:     usize,
    seconds_played: usize,
    _interval:      Interval,
}

impl App {
    fn reset(&mut self) {
        let cells = generate_cells(self.cells_width, self.cells_height, self.mines_count);
        self.reassign_cells(&cells);
        self.seconds_played = 0;
    }

    fn reassign_cells(&mut self, cells: &Vec<Cell>) {
        for (index, cell) in cells.iter().enumerate() {
            self.cells[index] = *cell;
        }
    }

    fn view_cell(&self, index: usize, cell: &Cell, link: &Scope<Self>) -> Html {
        let is_shown = {
            if cell.is_shown() { "clicked" } else { "" }
        };
        let is_mine = {
            if cell.is_mine() { "mine" } else { "" }
        };
        let value = cell.get_value_string();

        console::log!("is_mine, val", is_mine, value);
        html! {
            <td key={index}
                class={classes!("cell", is_shown, is_mine)}
                onclick={link.callback(move |_| Msg::Click(index))}
            >
                {value}
            </td>
        }
    }

}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        console::log!("Building app...");
        let callback = _ctx.link().callback(|_| Msg::Tick);
        let interval = Interval::new(1000, move || callback.emit(()));

        let (cells_width, cells_height, mines_count) = (100, 100, 10);
        let cells = generate_cells(cells_width, cells_height, mines_count);


        console::log!("Done");
        Self {
            active: false,
            face: Face::Happy,
            cells,
            cells_width,
            cells_height,
            mines_count,
            seconds_played: 0,
            _interval: interval,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Msg) -> bool {
        match msg {
            Msg::MouseDown => {
                if self.active { self.face = Face::Nervous; }
                true
            }
            Msg::Tick => {
                // if self.active {
                //     self.seconds_played += 1;
                //     true
                // } else {
                    false
                // }
            },
            Msg::Reset => {
                self.active = false;
                self.face = Face::Happy;
                self.reset();
                console::log!("Resetting...");
                true
            },
            Msg::Click(index) => {
                if !self.active { self.active = true; }

                let mut cell = self.cells[index];
                cell.handle_click();
                self.cells[index] = cell; // Need to reassign cell or its changes aren't saved

                if cell.is_mine() {
                    self.active = false;
                    self.face = Face::Dead;
                } else {
                    self.face = Face::Happy;
                }
                true
            },
            Msg::RightClick(index) => {
                let mut cell = self.cells[index];
                cell.cycle_display();
                true
            },
            // Msg::Chord(_) => todo!(),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
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
                            <span id="minesRemaining"></span>
                        </div>
                        <div id="resetButtonContainer" class="center">
                            <span id="resetButton" onclick={ctx.link().callback(move |_| Msg::Reset)}>{self.face.as_str()}</span>
                        </div>
                        <div id="timerContainer" class="counter right">
                            <span id="timer">{"000"}</span>
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

fn generate_cells(cells_width: usize, cells_height: usize, mines_count: usize) -> Vec<Cell> {
    let mut cells: Vec<Cell> = Vec::new();
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
        let (row, col) = get_row_col_from_index(index as isize, cells_width as isize, cells_height as isize);
        let neighbors = neighbors(row as isize, col as isize, cells_width as isize, cells_height as isize);
        let value = Cell::calculate_value(index, &neighbors, &mine_indicies);
        let cell = Cell::new(value);
        cells.push(cell);
    }

    cells
}

fn get_random_cell_index(cells_width: usize, cells_height: usize) -> usize {
    let mut rng = rand::thread_rng();
    rng.gen_range(0..(cells_width * cells_height))
}

fn neighbors(row: isize, col: isize, cells_width: isize, cells_height: isize) -> HashSet<usize> {
    let mut neighbors: HashSet<usize> = HashSet::new();

    neighbors.insert(get_index_from_row_col(row - 1, col - 1, cells_width, cells_height));
    neighbors.insert(get_index_from_row_col(row - 1, col, cells_width, cells_height));
    neighbors.insert(get_index_from_row_col(row - 1, col + 1, cells_width, cells_height));
    neighbors.insert(get_index_from_row_col(row, col - 1, cells_width, cells_height));
    neighbors.insert(get_index_from_row_col(row, col + 1, cells_width, cells_height));
    neighbors.insert(get_index_from_row_col(row + 1, col - 1, cells_width, cells_height));
    neighbors.insert(get_index_from_row_col(row + 1, col, cells_width, cells_height));
    neighbors.insert(get_index_from_row_col(row + 1, col + 1, cells_width, cells_height));

    neighbors
}

fn get_index_from_row_col(row: isize, col: isize, cells_width: isize, cells_height: isize) -> usize {
    let row = clamp(row, cells_height);
    let col = clamp(col, cells_width);

    row * cells_width as usize + col
}

fn get_row_col_from_index(index: isize, cells_width: isize, cells_height: isize) -> (usize, usize) {
    let row = index / cells_height;
    let col = index % cells_width;

    (row as usize, col as usize)
}

fn clamp(coord: isize, range: isize) -> usize {
    let result = if coord < 0 {
        0
    } else if coord > range {
        range
    } else {
        coord
    };
    result as usize
}

fn main() {
    yew::start_app::<App>();
}