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
        settings::{Difficulty, Settings}
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

pub struct Game {
    pub active: RwSignal<bool>,
    pub face: RwSignal<Face>,
    pub grid: RwSignal<CellGrid>,
    pub neighbors: RwSignal<Vec<HashSet<usize>>>,
    pub mine_indices: RwSignal<Vec<usize>>,
    pub selected_cell_index: RwSignal<Option<usize>>,
    pub first_clicked_mine_index: RwSignal<Option<usize>>,
    pub seconds_played: RwSignal<isize>,
    pub mouse_state: RwSignal<MouseState>,
    pub settings: RwSignal<Settings>,
    pub interval: RwSignal<Option<IntervalHandle>>,
}

impl Game {
    pub fn new() -> Self {
        let default_difficulty = Difficulty::Beginner;
        let default_grid_area = default_difficulty.dimensions().width()
            * default_difficulty.dimensions().height();
        let grid: CellGrid = (0..default_grid_area)
            .map(|_| {
                let mut cell = Cell::new_empty();
                cell.reset();
                cell
            })
            .collect::<Vec<_>>()
            .from_vec();
        let mine_indices: Vec<usize> = vec![0; default_difficulty.dimensions().mines()];
        let neighbors: Vec<HashSet<usize>> = (0..default_grid_area)
            .map(|_| HashSet::new())
            .collect();

        Game {
            active: create_rw_signal(true),
            face: create_rw_signal(Face::default()),
            grid: create_rw_signal(grid),
            neighbors: create_rw_signal(neighbors),
            mine_indices: create_rw_signal(mine_indices),
            selected_cell_index: create_rw_signal(None),
            first_clicked_mine_index: create_rw_signal(None),
            seconds_played: create_rw_signal(0),
            mouse_state: create_rw_signal(MouseState::default()),
            settings: create_rw_signal(Settings::default()),
            interval: create_rw_signal(None),
        }
    }

    pub fn grid_area(&self) -> usize {
        self.settings.with(|s| s.dimensions().width() * s.dimensions().height())
    }

    pub fn shown_cells_count(&self) -> usize {
        let temp = self.grid.with(|g|
            g.iter()
                .filter(|cell| cell.with(|c| c.is_shown()))
                .count()
        );
        log!("shown_cells_count {}", temp);
        temp
    }

    pub fn flagged_mines_count(&self) -> usize {
        let temp = self.grid.with(|g|
            g.iter()
                .filter(|cell| cell.with(|c| c.is_flagged()))
                .count()
        );
        log!("flagged_mines_count {}", temp);
        temp
    }

    pub fn mines_remaining(&self) -> isize {
        let temp = cmp::max(
            self.mine_indices.with(Vec::len) as isize - self.flagged_mines_count() as isize,
            -99
        );
        log!("mines_remaining {}", temp);
        temp
    }

    pub fn clear_interval(&self) {
        self.interval.update(|i| {
            if let Some(interval_handle) = i.take() {
                interval_handle.clear();
            }
        });
    }

    pub fn clear_cells(&self) {
        log!("clear_cells");
        batch(|| {
            self.grid.update(|g|
                g.iter()
                    .for_each(|cell| cell.update(|c| c.reset()))
            );
            log!("set_mines");
            self.mine_indices.update(|mine_inds|
                *mine_inds = (0..mine_inds.len())
                    .map(|_| 0)
                    .collect()
            );
        });
    }

    pub fn handle_reset(&self) {
        log!("handle_reset");
        self.clear_interval();
        log!("interval cleared");
        self.seconds_played.set(0);
        log!("seconds_played reset");
        self.clear_cells();
        log!("cells cleared");
        self.first_clicked_mine_index.set(None);
        log!("first_clicked_mine_index reset");
        self.active.set(true);
        log!("active set to true");
    }

    pub fn tick(&self) {
        self.seconds_played.update(|s| *s += 1);
    }

    pub fn start_interval(&self, closure: impl Fn() + 'static) {
        self.interval.update(|i| {
            if i.is_some() {
                log!("Interval already started");
                return;
            }
            *i = match set_interval_with_handle(closure, Duration::from_secs(1)) {
                Ok(interval_handle) => Some(interval_handle),
                Err(e) => {
                    error!("Error starting interval: {:?}", e);
                    None
                }
            }
        });
    }

    pub fn handle_change_size(&self, difficulty: Difficulty) {
        log!("current size: {:?}", self.settings.with(|s| s.dimensions()));
        self.settings.update(|s| s.set_difficulty(difficulty));
        log!("changing size to: {:?}", self.settings.with(|s| s.dimensions()));
        let dimensions = difficulty.dimensions();
        let area = self.grid_area();
        self.grid.set(
            (0..area)
                .map(|_| create_rw_signal(Cell::new_empty()))
                .collect()
        );
        self.neighbors.set(
            (0..area)
                .map(|_| HashSet::new())
                .collect()
        );
        self.mine_indices.set(
            vec![0; dimensions.mines()]
        );
        self.handle_reset();
    }

    pub fn get_row_col_from_index(&self, index: usize) -> (usize, usize) {
        self.settings.with(|s| {
            let row = index / s.dimensions().width();
            let col = index % s.dimensions().width();

            (row, col)
        })
    }

    pub fn get_index_from_row_col(&self, row: isize, col: isize) -> Option<usize> {
        self.settings.with(|s| {
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
        &self,
        index_clicked: usize,
        mine_index: usize,
        current_mine_indices: &HashSet<usize>,
        index_neighbors: &HashSet<usize>,
    ) -> bool {
        if current_mine_indices.contains(&mine_index) { return false; }
        if index_clicked == mine_index { return self.settings.with(|s| s.first_click_setting_is_any()); }
        if index_neighbors.contains(&mine_index) && self.settings.with(|s| s.first_click_setting_is_zero()) { return false; }
        true
    }

    pub fn calculate_neighbors(&self, index: usize) -> HashSet<usize> {
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

    pub fn get_random_cell_index(&self) -> usize {
        let mut rng = rand::thread_rng();
        rng.gen_range(0..self.grid_area())
    }

    pub fn generate_cells(&self, index_clicked: usize) {
        let mut new_cells: Vec<Cell> = vec![];
        let mut new_neighbors: Vec<HashSet<usize>> = vec![];
        let mut new_mine_indices: Vec<usize> = vec![];
        let mut current_mine_indices: HashSet<usize> = HashSet::new();
        let index_neighbors = self.calculate_neighbors(index_clicked);

        for _ in 0..self.mine_indices.with(Vec::len) {
            let mut i = self.get_random_cell_index();
            while !self.index_can_be_mine(index_clicked, i, &current_mine_indices, &index_neighbors) {
                i = self.get_random_cell_index();
            }
            current_mine_indices.insert(i);
        }
        log!("current_mine_indices: {:?}", current_mine_indices);

        for cell_index in 0..self.grid_area() {
            let neighboring_cells = self.calculate_neighbors(cell_index);
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
        self.grid.set(new_cells.from_vec());
        self.neighbors.set(new_neighbors);
        self.mine_indices.set(new_mine_indices);
    }

    pub fn click_all_mines(&self) {
    }

    pub fn click_neighboring_empty_cells(&self, index: usize) {
        for _i in self.neighbors.with(|n| n[index].clone()) {
            self.handle_click(index);
        }
    }

    pub fn handle_click(&self, index: usize) {
        if !self.active.get() { return; }

        log!("index: {}", index);
        if self.interval.with(Option::is_none) {
            self.generate_cells(index);
            // self.start_interval(move || self.tick());
        }

        let cell = self.grid.with(|g| g[index]);
        let set_cell = cell.write_only();
        let cell = cell();
        if cell.is_shown() || cell.is_flagged() {
            self.face.set(Face::Happy);
            return;
        }

        set_cell.update(|c| c.handle_click());

        if cell.is_mine() {
            let Some(selected_index) = self.selected_cell_index.get() else { return; };
            if self.first_clicked_mine_index.with(Option::is_none) &&
                (index == selected_index || self.neighbors.with(|n| n[selected_index].contains(&index)))
            {
                self.first_clicked_mine_index.set(Some(index));
                self.click_all_mines();
                self.active.set(false);
                self.face.set(Face::Dead);
                self.interval.set(None);
            }
            return;
        }

        self.face.set(Face::Happy);

        // Recursively click all neighboring cells if we clicked a 0
        if cell.is_zero() { self.click_neighboring_empty_cells(index); }
        // self.check_for_win();
    }
}
