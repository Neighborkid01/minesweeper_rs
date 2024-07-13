use leptos::*;
use leptos::logging::*;
use leptos_dom::helpers::IntervalHandle;
use rand::Rng;
use std::{cmp, collections::HashSet, time::Duration};
use crate::{
    CellGrid,
    models::{
        cell::Cell as Cell,
        face::Face,
        mouse_state::MouseState,
        settings::{Difficulty, Dimensions, Settings}
    },
};

pub trait FromVec<T> {
    fn from_vec(&self) -> CellGrid;
}
impl FromVec<Cell> for Vec<Cell> {
    fn from_vec(&self) -> CellGrid {
        self.iter()
            .map(|cell| create_rw_signal(cell.clone()))
            .collect()
    }
}

pub fn shown_cells_count(grid: ReadSignal<CellGrid>) -> usize {
    let temp = grid.with(|g|
        g.iter()
            .filter(|cell| cell.with(|c| c.is_shown()))
            .count()
    );
    log!("shown_cells_count {}", temp);
    temp
}

pub fn flagged_mines_count(grid: ReadSignal<CellGrid>) -> usize {
    let temp = grid.with(|g|
        g.iter()
            .filter(|cell| cell.with(|c| c.is_flagged()))
            .count()
    );
    log!("flagged_mines_count {}", temp);
    temp
}

pub fn mines_remaining(grid: ReadSignal<CellGrid>, mine_indices: ReadSignal<Vec<usize>>) -> isize {
    let temp = cmp::max(
        mine_indices.with(|mine_inds| mine_inds.len()) as isize - flagged_mines_count(grid) as isize,
        -99
    );
    log!("mines_remaining {}", temp);
    temp
}

pub fn clear_interval(set_interval: WriteSignal<Option<IntervalHandle>>) {
    set_interval.update(|i| {
        if let Some(interval_handle) = i.take() {
            interval_handle.clear();
        }
    });
}

pub fn clear_cells(set_grid: WriteSignal<CellGrid>, set_mine_indices: WriteSignal<Vec<usize>>) {
    log!("clear_cells");
    set_grid.update(|grid|
        grid.iter_mut()
            .for_each(|cell| cell.update(|c| c.reset()))
    );
    log!("set_mines");
    set_mine_indices.update(|mine_inds|
        *mine_inds = (0..mine_inds.len())
            .map(|_| 0)
            .collect()
    );
}

pub fn handle_reset(
    set_grid: WriteSignal<CellGrid>,
    set_mine_indices: WriteSignal<Vec<usize>>,
    set_interval: WriteSignal<Option<IntervalHandle>>,
    set_seconds_played: WriteSignal<isize>,
    set_first_clicked_mine_index: WriteSignal<Option<usize>>,
    set_active: WriteSignal<bool>
) {
    log!("handle_reset");
    clear_interval(set_interval);
    log!("interval cleared");
    set_seconds_played(0);
    log!("seconds_played reset");
    clear_cells(set_grid, set_mine_indices);
    log!("cells cleared");
    set_first_clicked_mine_index(None);
    log!("first_clicked_mine_index reset");
    set_active(true);
    log!("active set to true");
}

pub fn tick(set_seconds_played: WriteSignal<isize>) {
    set_seconds_played.update(|s| *s += 1);
}

pub fn start_interval(set_interval: WriteSignal<Option<IntervalHandle>>, set_seconds_played: WriteSignal<isize>) {
    set_interval.update(|i| {
        if i.is_some() {
            log!("Interval already started");
            return;
        }
        *i = match set_interval_with_handle(move || tick(set_seconds_played), Duration::from_secs(1)) {
            Ok(interval_handle) => Some(interval_handle),
            Err(e) => {
                error!("Error starting interval: {:?}", e);
                None
            }
        }
    });
}

pub fn handle_change_size(
    difficulty: Difficulty,
    settings: ReadSignal<Settings>,
    set_settings: WriteSignal<Settings>,
    set_grid: WriteSignal<CellGrid>,
    grid_area: &impl Fn() -> usize,
    set_neighbors: WriteSignal<Vec<HashSet<usize>>>,
    set_mine_indices: WriteSignal<Vec<usize>>,
    set_interval: WriteSignal<Option<IntervalHandle>>,
    set_seconds_played: WriteSignal<isize>,
    set_first_clicked_mine_index: WriteSignal<Option<usize>>,
    set_active: WriteSignal<bool>
) {
    log!("current size: {:?}", settings().dimensions());
    set_settings.update(|s| s.set_difficulty(difficulty));
    log!("changing size to: {:?}", settings().dimensions());
    let dimensions = difficulty.dimensions();
    set_grid(
        (0..grid_area())
            .map(|_| create_rw_signal(Cell::new_empty()))
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
    handle_reset(set_grid, set_mine_indices, set_interval, set_seconds_played, set_first_clicked_mine_index, set_active);
}

pub fn get_row_col_from_index(index: usize, settings: ReadSignal<Settings>) -> (usize, usize) {
    settings.with(|s| {
        let row = index / s.dimensions().width();
        let col = index % s.dimensions().width();

        (row, col)
    })
}

pub fn get_index_from_row_col(row: isize, col: isize, settings: ReadSignal<Settings>) -> Option<usize> {
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
}

pub fn index_can_be_mine(
    index_clicked: usize,
    mine_index: usize,
    current_mine_indices: &HashSet<usize>,
    index_neighbors: &HashSet<usize>,
    settings: ReadSignal<Settings>,
) -> bool {
    if current_mine_indices.contains(&mine_index) { return false; }
        if index_clicked == mine_index { return settings.with(|s| s.first_click_setting_is_any()); }
        if index_neighbors.contains(&mine_index) && settings.with(|s| s.first_click_setting_is_zero()) { return false; }
        true
}

pub fn calculate_neighbors(index: usize, settings: ReadSignal<Settings>) -> HashSet<usize> {
    let (row, col) = get_row_col_from_index(index, settings);
    let mut neighbors: HashSet<usize> = HashSet::new();
    let r = row as isize;
    let c = col as isize;

    if let Some(n) = get_index_from_row_col(r - 1, c - 1, settings)  { neighbors.insert(n); }
    if let Some(n) = get_index_from_row_col(r - 1, c, settings)      { neighbors.insert(n); }
    if let Some(n) = get_index_from_row_col(r - 1, c + 1, settings)  { neighbors.insert(n); }
    if let Some(n) = get_index_from_row_col(r, c - 1, settings)      { neighbors.insert(n); }
    if let Some(n) = get_index_from_row_col(r, c + 1, settings)      { neighbors.insert(n); }
    if let Some(n) = get_index_from_row_col(r + 1, c - 1, settings)  { neighbors.insert(n); }
    if let Some(n) = get_index_from_row_col(r + 1, c, settings)      { neighbors.insert(n); }
    if let Some(n) = get_index_from_row_col(r + 1, c + 1, settings)  { neighbors.insert(n); }

    neighbors
}

pub fn get_random_cell_index(grid_area: &impl Fn() -> usize) -> usize {
    let mut rng = rand::thread_rng();
    rng.gen_range(0..grid_area())
}

pub fn generate_cells(
    index_clicked: usize,
    settings: ReadSignal<Settings>,
    mine_indices: ReadSignal<Vec<usize>>,
    grid_area: &impl Fn() -> usize,
    set_grid: WriteSignal<CellGrid>,
    set_neighbors: WriteSignal<Vec<HashSet<usize>>>,
    set_mine_indices: WriteSignal<Vec<usize>>,
) {
    let mut new_cells: Vec<Cell> = vec![];
    let mut new_neighbors: Vec<HashSet<usize>> = vec![];
    let mut new_mine_indices: Vec<usize> = vec![];
    let mut current_mine_indices: HashSet<usize> = HashSet::new();
    let index_neighbors = calculate_neighbors(index_clicked, settings);

    for _ in 0..mine_indices.with(|mine_inds| mine_inds.len()) {
        let mut i = get_random_cell_index(grid_area);
        while !index_can_be_mine(index_clicked, i, &current_mine_indices, &index_neighbors, settings) {
            i = get_random_cell_index(grid_area);
        }
        current_mine_indices.insert(i);
    }
    log!("current_mine_indices: {:?}", current_mine_indices);

    for cell_index in 0..grid_area() {
        let neighboring_cells = calculate_neighbors(cell_index, settings);
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
}

pub fn click_all_mines() {
}

pub fn click_neighboring_empty_cells(index: usize, neighbors: ReadSignal<Vec<HashSet<usize>>>) {
    for _i in neighbors.with(|n| n[index].clone()) {
        // handle_click(index);
    }
}

pub fn handle_click(
    index: usize,
    active: ReadSignal<bool>,
    interval: ReadSignal<Option<IntervalHandle>>,
    settings: ReadSignal<Settings>,
    mine_indices: ReadSignal<Vec<usize>>,
    grid_area: &impl Fn() -> usize,
    grid: ReadSignal<CellGrid>,
    set_grid: WriteSignal<CellGrid>,
    neighbors: ReadSignal<Vec<HashSet<usize>>>,
    set_neighbors: WriteSignal<Vec<HashSet<usize>>>,
    set_mine_indices: WriteSignal<Vec<usize>>,
    set_interval: WriteSignal<Option<IntervalHandle>>,
    set_seconds_played: WriteSignal<isize>,
    set_first_clicked_mine_index: WriteSignal<Option<usize>>,
    set_active: WriteSignal<bool>,
    set_face: WriteSignal<Face>,
    selected_cell_index: ReadSignal<Option<usize>>,
    first_clicked_mine_index: ReadSignal<Option<usize>>,
) {
    if !active() { return; }

        log!("index: {}", index);
        if interval.with(|i| i.is_none()) {
            generate_cells(index, settings, mine_indices, &grid_area, set_grid, set_neighbors, set_mine_indices);
            start_interval(set_interval, set_seconds_played);
        }

        let cell = grid.with(|g| g[index]);
        let set_cell = cell.write_only();
        let cell = cell();
        if cell.is_shown() || cell.is_flagged() {
            set_face(Face::Happy);
            return;
        }

        set_cell.update(|c| c.handle_click());

        if cell.is_mine() {
            let Some(selected_index) = selected_cell_index() else { return; };
            if first_clicked_mine_index.with(|i| i.is_none()) &&
                (index == selected_index || neighbors.with(|n| n[selected_index].contains(&index)))
            {
                set_first_clicked_mine_index(Some(index));
                click_all_mines();
                set_active(false);
                set_face(Face::Dead);
                set_interval(None);
            }
            return;
        }

        set_face(Face::Happy);

        // Recursively click all neighboring cells if we clicked a 0
        if cell.is_zero() { click_neighboring_empty_cells(index, neighbors); }
        // self.check_for_win();
}
