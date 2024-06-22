mod components;
mod models;

use leptos::*;
use leptos::logging::*;
use components::{
    cell_grid::CellGrid,
    counter::Counter,
    difficulty_option::DifficultyOption,
};
use leptos_dom::helpers::IntervalHandle;
use models::{
    face::Face,
    cell::Cell as Cell,
    mouse_state::MouseState,
    settings::{Difficulty, Settings, Dimensions},
};
use wasm_bindgen::JsCast;
use web_sys::{Element, MouseEvent};
use rand::Rng;
use std::{cmp, collections::HashSet, time::Duration, vec};

type CellGrid = Vec<(ReadSignal<Cell>, WriteSignal<Cell>)>;
trait FromVec<T> {
    fn from_vec(&self) -> CellGrid;
}
impl FromVec<Cell> for Vec<Cell> {
    fn from_vec(&self) -> CellGrid {
        self.iter()
            .map(|cell| create_signal(cell.clone()))
            .collect()
    }
}

#[component]
fn App() -> impl IntoView {

    let default_difficulty = Difficulty::Beginner;
    let default_grid_area = default_difficulty.dimensions().width()
        * default_difficulty.dimensions().height();
    let grid: CellGrid = (0..default_grid_area)
        .map(|_| Cell::new_empty())
        .collect::<Vec<_>>()
        .from_vec();
    let mine_indices: Vec<usize> = vec![0; default_difficulty.dimensions().mines()];
    let neighbors: Vec<HashSet<usize>> = vec![];
    let (active, set_active) = create_signal(false);
    let (face, set_face) = create_signal(Face::default());
    let (grid, set_grid) = create_signal(grid);
    let (neighbors, set_neighbors) = create_signal(neighbors);
    let (mine_indices, set_mine_indices) = create_signal(mine_indices);
    let (shown_cells_count, set_shown_cells_count) = create_signal(0); // should be derived
    let (selected_cell_index, set_selected_cell_index) = create_signal(None::<usize>);
    let (first_clicked_mine_index, set_first_clicked_mine_index) = create_signal(None::<usize>);
    let (seconds_played, set_seconds_played) = create_signal(0);
    let (mouse_state, set_mouse_state) = create_signal(MouseState::default());
    let (settings, set_settings) = create_signal(Settings::default());
    let (interval, set_interval) = create_signal(None::<leptos_dom::helpers::IntervalHandle>);
    let grid_area = move || settings.with(|s| s.dimensions().width() * s.dimensions().height());

    let clear_interval = move || {
        set_interval.update(|i| {
            if let Some(interval_handle) = i.take() {
                interval_handle.clear();
            }
        });
    };

    let clear_cells = move || {
        set_grid.update(|grid| {
            grid.iter_mut()
                .for_each(|(_, set_cell)| {
                    set_cell.update(|cell| cell.reset());
                });
        });
        set_mine_indices.update(|mine_inds|
            *mine_inds = (0..mine_inds.len())
                .map(|_| 0)
                .collect()
        );
    };

    let handle_reset = move || {
        clear_interval();
        set_seconds_played(0);
        set_shown_cells_count(0); // should be derived
        clear_cells();
        set_first_clicked_mine_index(None);
        // set_active(true);
    };

    let tick = move || {
        set_seconds_played.update(|s| *s += 1);
    };

    let start_interval = move || {
        set_interval.update(|i| {
            if i.is_some() {
                log!("Interval already started");
                return;
            }
            *i = match set_interval_with_handle(tick, Duration::from_secs(1)) {
                Ok(interval_handle) => Some(interval_handle),
                Err(e) => {
                    error!("Error starting interval: {:?}", e);
                    None
                }
            }
        });
    };

    let handle_change_size = move |difficulty: Difficulty| {
        log!("current size: {:?}", settings().dimensions());
        set_settings.update(|s| { s.set_difficulty(difficulty) });
        log!("changing size to: {:?}", settings().dimensions());
        let dimensions = difficulty.dimensions();
        set_grid(
            (0..grid_area())
                .map(|_| create_signal(Cell::new_empty()))
                .collect()
        );
        set_neighbors(
            (0..grid_area())
                .map(|_| HashSet::new())
                .collect()
        );
        set_mine_indices.set(
            vec![0; dimensions.mines()]
        );
        handle_reset();
    };

    let get_row_col_from_index = move |index: usize| -> (usize, usize) {
        settings.with(|s| {
            let row = index / s.dimensions().width();
            let col = index % s.dimensions().width();

            (row, col)
        })
    };

    let get_index_from_row_col = move |row: isize, col: isize| -> Option<usize> {
        settings.with(|s| {
            let u_row = row as usize;
            let u_col = col as usize;
            if row >= 0 && u_row < s.dimensions().height() &&
                col >= 0 && u_col < s.dimensions().width()
            {
                Some((u_row * s.dimensions().width()) + u_col)
            } else {
                None
            }
        })
    };

    let index_can_be_mine = move |index_clicked: usize, mine_index: usize, current_mine_indices: &HashSet<usize>, index_neighbors: &HashSet<usize>| -> bool {
        if current_mine_indices.contains(&mine_index) { return false; }
        if index_clicked == mine_index { return settings.with(|s| s.first_click_setting_is_any()); }
        if index_neighbors.contains(&mine_index) && settings.with(|s| s.first_click_setting_is_zero()) { return false; }
        true
    };

    let calculate_neighbors = move |index: usize| -> HashSet<usize> {
        let (row, col) = get_row_col_from_index(index);
        let mut neighbors: HashSet<usize> = HashSet::new();
        let r = row as isize;
        let c = col as isize;

        if let Some(n) = get_index_from_row_col(r - 1, c - 1)  { neighbors.insert(n); }
        if let Some(n) = get_index_from_row_col(r - 1, c)      { neighbors.insert(n); }
        if let Some(n) = get_index_from_row_col(r - 1, c + 1)  { neighbors.insert(n); }
        if let Some(n) = get_index_from_row_col(r, c - 1)      { neighbors.insert(n); }
        if let Some(n) = get_index_from_row_col(r, c + 1)      { neighbors.insert(n); }
        if let Some(n) = get_index_from_row_col(r + 1, c - 1)  { neighbors.insert(n); }
        if let Some(n) = get_index_from_row_col(r + 1, c)      { neighbors.insert(n); }
        if let Some(n) = get_index_from_row_col(r + 1, c + 1)  { neighbors.insert(n); }

        neighbors
    };

    let get_random_cell_index = move || -> usize {
        let mut rng = rand::thread_rng();
        rng.gen_range(0..grid_area())
    };

    let generate_cells = move |index_clicked: usize| {
        let mut new_cells: Vec<Cell> = vec![];
        let mut new_neighbors: Vec<HashSet<usize>> = vec![];
        let mut new_mine_indices: Vec<usize> = vec![];
        let mut current_mine_indices: HashSet<usize> = HashSet::new();
        let index_neighbors = calculate_neighbors(index_clicked);

        for _ in 0..mine_indices.with(|mine_inds| mine_inds.len()) {
            let mut i = get_random_cell_index();
            while !index_can_be_mine(index_clicked, i, &current_mine_indices, &index_neighbors) {
                i = get_random_cell_index();
            }
            current_mine_indices.insert(i);
        }
        log!("current_mine_indices: {:?}", current_mine_indices);

        for cell_index in 0..grid_area() {
            let neighboring_cells = calculate_neighbors(cell_index);
            let neighboring_mines = if current_mine_indices.contains(&cell_index) {
                None
            } else {
                Some(neighboring_cells.intersection(&current_mine_indices).count())
            };
            let cell = Cell::new(neighboring_mines);
            new_cells.push(cell);
            new_neighbors.push(neighboring_cells);
            if current_mine_indices.contains(&cell_index) { new_mine_indices.push(cell_index); }
        }

        log!("new_cells: {:?}", new_cells);
        set_grid(new_cells.from_vec());
        set_neighbors(new_neighbors);
        set_mine_indices(new_mine_indices);
    };

    let handle_click = move |_e: MouseEvent, index: usize| {
        log!("index: {}", index);
        if interval.with(|i| i.is_none()) {
            // set_active(true);
            start_interval();
            generate_cells(index);
        }
    };

    log!("App!");
    view! {
        <div class="container no-select">
            <div class="settings">
                <DifficultyOption
                    difficulty_to_display={Difficulty::Beginner}
                    settings
                    on_difficulty_selected=handle_change_size
                />
                <DifficultyOption
                    difficulty_to_display={Difficulty::Intermediate}
                    settings
                    on_difficulty_selected=handle_change_size
                />
                <DifficultyOption
                    difficulty_to_display={Difficulty::Expert}
                    settings
                    on_difficulty_selected=handle_change_size
                />
                <DifficultyOption
                    difficulty_to_display={Difficulty::Custom(Dimensions::default())}
                    settings
                    on_difficulty_selected=handle_change_size
                />
            </div>

            <div class="header">
                <Counter value=Signal::derive(move || {42} /* mines_remaining */ )/>
                <div id="resetButtonContainer" class="center">
                    <span id="resetButton" on:click=move |_| handle_reset()>
                        { move || face().to_str() }
                    </span>
                </div>
                <Counter value=seconds_played />
            </div>

            <div class="board-container">
                <table id="board" class="board"
                    on:contextmenu=move |e: MouseEvent| e.prevent_default()
                >
                    <CellGrid
                        grid
                        settings
                        handle_click
                    />
                </table>
            </div>
        </div>
    }
}

fn main() {
    mount_to_body(|| view! { <App /> })
}
