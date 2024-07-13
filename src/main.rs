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
    cell::Cell as Cell,
    face::Face,
    game::*,
    mouse_state::MouseState,
    settings::{Difficulty, Settings, Dimensions},
};
use wasm_bindgen::JsCast;
use web_sys::{Element, MouseEvent};
use rand::Rng;
use std::{cmp, collections::HashSet, time::Duration, vec};

type CellGrid = Vec<RwSignal<Cell>>;

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
    let (selected_cell_index, set_selected_cell_index) = create_signal(None::<usize>);
    let (first_clicked_mine_index, set_first_clicked_mine_index) = create_signal(None::<usize>);
    let (seconds_played, set_seconds_played) = create_signal(0);
    let (mouse_state, set_mouse_state) = create_signal(MouseState::default());
    let (settings, set_settings) = create_signal(Settings::default());
    let (interval, set_interval) = create_signal(None::<leptos_dom::helpers::IntervalHandle>);
    let grid_area = move || settings.with(|s| s.dimensions().width() * s.dimensions().height());

    // let shown_cells_count = move || {
    //     let temp = grid.with(|g|
    //         g.iter()
    //             .filter(|cell| cell.with(|c| c.is_shown()))
    //             .count()
    //     );
    //     log!("shown_cells_count {}", temp);
    //     temp
    // };

    // let flagged_mines_count = move || {
    //     let temp = grid.with(|g|
    //         g.iter()
    //             .filter(|cell| cell.with(|c| c.is_flagged()))
    //             .count()
    //     );
    //     log!("flagged_mines_count {}", temp);
    //     temp
    // };

    // let mines_remaining = move || {
    //     let temp = cmp::max(
    //         mine_indices.with(|mine_inds| mine_inds.len()) as isize - flagged_mines_count(grid) as isize,
    //         -99
    //     );
    //     log!("mines_remaining {}", temp);
    //     temp
    // };

    // let clear_interval = move || {
    //     set_interval.update(|i| {
    //         if let Some(interval_handle) = i.take() {
    //             interval_handle.clear();
    //         }
    //     });
    // };

    // let clear_cells = move || {
    //     log!("clear_cells");
    //     set_grid.update(|grid|
    //         grid.iter_mut()
    //             .for_each(|cell|
    //                 cell.update(|c| c.reset())
    //             )
    //     );
    //     log!("set_mines");
    //     set_mine_indices.update(|mine_inds|
    //         *mine_inds = (0..mine_inds.len())
    //             .map(|_| 0)
    //             .collect()
    //     );
    // };

    // let handle_reset = move || {
    //     log!("handle_reset");
    //     clear_interval(set_interval);
    //     log!("interval cleared");
    //     set_seconds_played(0);
    //     log!("seconds_played reset");
    //     clear_cells(set_grid, set_mine_indices);
    //     log!("cells cleared");
    //     set_first_clicked_mine_index(None);
    //     log!("first_clicked_mine_index reset");
    //     set_active(true);
    //     log!("active set to true");
    // };

    // let tick = move || {
    //     set_seconds_played.update(|s| *s += 1);
    // };

    // let start_interval = move || {
    //     set_interval.update(|i| {
    //         if i.is_some() {
    //             log!("Interval already started");
    //             return;
    //         }
    //         *i = match set_interval_with_handle(tick, Duration::from_secs(1)) {
    //             Ok(interval_handle) => Some(interval_handle),
    //             Err(e) => {
    //                 error!("Error starting interval: {:?}", e);
    //                 None
    //             }
    //         }
    //     });
    // };

    // let handle_change_size = move |difficulty: Difficulty| {
    //     log!("current size: {:?}", settings().dimensions());
    //     set_settings.update(|s| s.set_difficulty(difficulty));
    //     log!("changing size to: {:?}", settings().dimensions());
    //     let dimensions = difficulty.dimensions();
    //     set_grid(
    //         (0..grid_area())
    //             .map(|_| create_rw_signal(Cell::new_empty()))
    //             .collect()
    //     );
    //     set_neighbors(
    //         (0..grid_area())
    //             .map(|_| HashSet::new())
    //             .collect()
    //     );
    //     set_mine_indices.set(
    //         vec![0; dimensions.mines()]
    //     );
    //     handle_reset(set_grid, set_mine_indices, set_interval, set_seconds_played, set_first_clicked_mine_index, set_active);
    // };

    // let get_row_col_from_index = move |index: usize| -> (usize, usize) {
    //     settings.with(|s| {
    //         let row = index / s.dimensions().width();
    //         let col = index % s.dimensions().width();

    //         (row, col)
    //     })
    // };

    // let get_index_from_row_col = move |row: isize, col: isize| -> Option<usize> {
    //     settings.with(|s| {
    //         let u_row = row as usize;
    //         let u_col = col as usize;
    //         if row >= 0 && u_row < s.dimensions().height() &&
    //             col >= 0 && u_col < s.dimensions().width()
    //         {
    //             Some((u_row * s.dimensions().width()) + u_col)
    //         } else {
    //             None
    //         }
    //     })
    // };

    // let index_can_be_mine = move |
    //     index_clicked: usize,
    //     mine_index: usize,
    //     current_mine_indices: &HashSet<usize>,
    //     index_neighbors: &HashSet<usize>
    // | -> bool {
    //     if current_mine_indices.contains(&mine_index) { return false; }
    //     if index_clicked == mine_index { return settings.with(|s| s.first_click_setting_is_any()); }
    //     if index_neighbors.contains(&mine_index) && settings.with(|s| s.first_click_setting_is_zero()) { return false; }
    //     true
    // };

    // let calculate_neighbors = move |index: usize| -> HashSet<usize> {
    //     let (row, col) = get_row_col_from_index(index, settings);
    //     let mut neighbors: HashSet<usize> = HashSet::new();
    //     let r = row as isize;
    //     let c = col as isize;

    //     if let Some(n) = get_index_from_row_col(r - 1, c - 1, settings)  { neighbors.insert(n); }
    //     if let Some(n) = get_index_from_row_col(r - 1, c, settings)      { neighbors.insert(n); }
    //     if let Some(n) = get_index_from_row_col(r - 1, c + 1, settings)  { neighbors.insert(n); }
    //     if let Some(n) = get_index_from_row_col(r, c - 1, settings)      { neighbors.insert(n); }
    //     if let Some(n) = get_index_from_row_col(r, c + 1, settings)      { neighbors.insert(n); }
    //     if let Some(n) = get_index_from_row_col(r + 1, c - 1, settings)  { neighbors.insert(n); }
    //     if let Some(n) = get_index_from_row_col(r + 1, c, settings)      { neighbors.insert(n); }
    //     if let Some(n) = get_index_from_row_col(r + 1, c + 1, settings)  { neighbors.insert(n); }

    //     neighbors
    // };

    // let get_random_cell_index = move || -> usize {
    //     let mut rng = rand::thread_rng();
    //     rng.gen_range(0..grid_area())
    // };

    // let generate_cells = move |index_clicked: usize| {
    //     let mut new_cells: Vec<Cell> = vec![];
    //     let mut new_neighbors: Vec<HashSet<usize>> = vec![];
    //     let mut new_mine_indices: Vec<usize> = vec![];
    //     let mut current_mine_indices: HashSet<usize> = HashSet::new();
    //     let index_neighbors = calculate_neighbors(index_clicked, settings);

    //     for _ in 0..mine_indices.with(|mine_inds| mine_inds.len()) {
    //         let mut i = get_random_cell_index(&grid_area);
    //         while !index_can_be_mine(index_clicked, i, &current_mine_indices, &index_neighbors, settings) {
    //             i = get_random_cell_index(&grid_area);
    //         }
    //         current_mine_indices.insert(i);
    //     }
    //     log!("current_mine_indices: {:?}", current_mine_indices);

    //     for cell_index in 0..grid_area() {
    //         let neighboring_cells = calculate_neighbors(cell_index, settings);
    //         let neighboring_mines = if current_mine_indices.contains(&cell_index) {
    //             None
    //         } else {
    //             Some(neighboring_cells.intersection(&current_mine_indices).count())
    //         };
    //         let cell = Cell::new(neighboring_mines);
    //         new_cells.push(cell);
    //         new_neighbors.push(neighboring_cells);
    //         if current_mine_indices.contains(&cell_index) { new_mine_indices.push(cell_index); }
    //     }

    //     log!("new_cells: {:?}", new_cells);
    //     set_grid(new_cells.from_vec());
    //     set_neighbors(new_neighbors);
    //     set_mine_indices(new_mine_indices);
    // };

    // let click_all_mines = move || {};

    // let handle_click: impl Fn(usize);

    // let click_neighboring_empty_cells = move |index: usize| {
    //     // let neighbors = self.neighbors[index].clone();
    //     for _i in neighbors.with(|n| n[index].clone()) {
    //         // handle_click(index);
    //     }
    // };

    // let handle_click = move |index: usize| {
    //     if !active() { return; }

    //     log!("index: {}", index);
    //     if interval.with(|i| i.is_none()) {
    //         generate_cells(index, settings, mine_indices, &grid_area, set_grid, set_neighbors, set_mine_indices);
    //         start_interval(set_interval, set_seconds_played);
    //     }

    //     let cell = grid.with(|g| g[index]);
    //     let set_cell = cell.write_only();
    //     let cell = cell();
    //     if cell.is_shown() || cell.is_flagged() {
    //         set_face(Face::Happy);
    //         return;
    //     }

    //     set_cell.update(|c| c.handle_click());

    //     if cell.is_mine() {
    //         let Some(selected_index) = selected_cell_index() else { return; };
    //         if first_clicked_mine_index.with(|i| i.is_none()) &&
    //             (index == selected_index || neighbors.with(|n| n[selected_index].contains(&index)))
    //         {
    //             set_first_clicked_mine_index(Some(index));
    //             click_all_mines();
    //             set_active(false);
    //             set_face(Face::Dead);
    //             set_interval(None);
    //         }
    //         return;
    //     }

    //     set_face(Face::Happy);

    //     // Recursively click all neighboring cells if we clicked a 0
    //     if cell.is_zero() { click_neighboring_empty_cells(index, neighbors); }
    //     // self.check_for_win();
    // };

    log!("App!");
    view! {
        <div class="container no-select">
            <div class="settings">
                <DifficultyOption
                    difficulty_to_display={Difficulty::Beginner}
                    settings
                    on_difficulty_selected=move |difficulty: Difficulty| handle_change_size(difficulty, settings, set_settings, set_grid, &grid_area, set_neighbors, set_mine_indices, set_interval, set_seconds_played, set_first_clicked_mine_index, set_active)
                />
                <DifficultyOption
                    difficulty_to_display={Difficulty::Intermediate}
                    settings
                    on_difficulty_selected=move |difficulty: Difficulty| handle_change_size(difficulty, settings, set_settings, set_grid, &grid_area, set_neighbors, set_mine_indices, set_interval, set_seconds_played, set_first_clicked_mine_index, set_active)
                />
                <DifficultyOption
                    difficulty_to_display={Difficulty::Expert}
                    settings
                    on_difficulty_selected=move |difficulty: Difficulty| handle_change_size(difficulty, settings, set_settings, set_grid, &grid_area, set_neighbors, set_mine_indices, set_interval, set_seconds_played, set_first_clicked_mine_index, set_active)
                />
                <DifficultyOption
                    difficulty_to_display={Difficulty::Custom(Dimensions::default())}
                    settings
                    on_difficulty_selected=move |difficulty: Difficulty| handle_change_size(difficulty, settings, set_settings, set_grid, &grid_area, set_neighbors, set_mine_indices, set_interval, set_seconds_played, set_first_clicked_mine_index, set_active)
                />
            </div>

            <div class="header">
                <Counter value={ move || mines_remaining(grid, mine_indices) } />
                <div id="resetButtonContainer" class="center">
                    <span id="resetButton" on:click=move |_| handle_reset(set_grid, set_mine_indices, set_interval, set_seconds_played, set_first_clicked_mine_index, set_active)>
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
                        handle_click={move |index: usize| handle_click(index, active, interval, settings, mine_indices, &grid_area, grid, set_grid, neighbors, set_neighbors, set_mine_indices, set_interval, set_seconds_played, set_first_clicked_mine_index, set_active, set_face, selected_cell_index, first_clicked_mine_index) }
                    />
                </table>
            </div>
        </div>
    }
}

fn main() {
    mount_to_body(|| view! { <App /> })
}
