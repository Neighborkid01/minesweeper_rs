use leptos::*;
use leptos::logging::*;
use leptos_dom::helpers::IntervalHandle;
use rand::Rng;
use std::{cmp, collections::HashSet, time::Duration};
use web_sys::MouseEvent;
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

    fn grid_area(&self) -> usize {
        self.settings.with(|s| s.dimensions().width() * s.dimensions().height())
    }

    fn shown_cells_count(&self) -> usize {
        self.grid.with(|g|
            g.iter()
                .filter(|cell| cell.with(|c| c.is_shown()))
                .count()
        )
    }

    fn flagged_mines_count(&self) -> usize {
        self.grid.with(|g|
            g.iter()
                .filter(|cell| cell.with(|c| c.is_flagged()))
                .count()
        )
    }

    pub fn mines_remaining(&self) -> isize {
        cmp::max(
            self.mine_indices.with(Vec::len) as isize - self.flagged_mines_count() as isize,
            -99
        )
    }

    fn clear_interval(&self) {
        self.interval.update(|i| {
            if let Some(interval_handle) = i.take() {
                interval_handle.clear();
            }
        });
    }

    fn clear_cells(&self) {
        batch(|| {
            self.grid.update(|g|
                g.iter()
                    .for_each(|cell| cell.update(|c| c.reset()))
            );
            self.mine_indices.update(|mine_inds|
                *mine_inds = (0..mine_inds.len())
                    .map(|_| 0)
                    .collect()
            );
        });
    }

    pub fn handle_reset(&self) {
        self.clear_interval();
        self.seconds_played.set(0);
        self.clear_cells();
        self.first_clicked_mine_index.set(None);
        self.mouse_state.set(MouseState::default());
        self.selected_cell_index.set(None);
        self.active.set(true);
    }

    fn start_interval(&self, tick: impl Fn() + 'static) {
        self.interval.update(|i| {
            if i.is_some() { return; }
            *i = match set_interval_with_handle(tick, Duration::from_secs(1)) {
                Ok(interval_handle) => Some(interval_handle),
                Err(e) => {
                    error!("Error starting interval: {:?}", e);
                    None
                }
            }
        });
    }

    pub fn handle_change_size(&self, difficulty: Difficulty) {
        self.settings.update(|s| s.set_difficulty(difficulty));
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

    fn get_row_col_from_index(&self, index: usize) -> (usize, usize) {
        self.settings.with(|s| {
            let row = index / s.dimensions().width();
            let col = index % s.dimensions().width();

            (row, col)
        })
    }

    fn get_index_from_row_col(&self, row: isize, col: isize) -> Option<usize> {
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

    fn index_can_be_mine(
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
        rng.gen_range(0..self.grid_area())
    }

    fn generate_cells(&self, index_clicked: usize) {
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

        self.grid.set(new_cells.from_vec());
        self.neighbors.set(new_neighbors);
        self.mine_indices.set(new_mine_indices);
    }

    fn click_all_mines(&self) {
        batch(|| {
            let (mine_inds, g, first_mine_index) = (self.mine_indices, self.grid, self.first_clicked_mine_index);
            with! { |mine_inds, g, first_mine_index|
                for &index in mine_inds.iter() {
                    g[index].update(|c| {
                        c.handle_click();
                        if *first_mine_index == Some(index) { c.mark_as_first_clicked_mine(); }
                    });
                }
            }
        });
    }

    fn click_neighboring_empty_cells(&self, index: usize) {
        let mut to_reveal = HashSet::new();
        let mut queue = vec![index];

        while let Some(current_index) = queue.pop() {
            if to_reveal.contains(&current_index) { continue; }
            to_reveal.insert(current_index);

            let is_zero = self.grid.with(|g| g[current_index].with(|c| c.is_zero()));
            if !is_zero { continue; }

            let neighbors = self.neighbors.with(|n| n[current_index].clone());
            for &neighbor in neighbors.iter() {
                let should_process = self.grid.with(|g| {
                    let cell = &g[neighbor];
                    !cell.with(|c| c.is_shown() || c.is_flagged())
                });
                if should_process && !to_reveal.contains(&neighbor) { queue.push(neighbor); }
            }
        }

        batch(|| {
            self.grid.with(|g| {
                for cell_index in to_reveal {
                    g[cell_index].update(|c| c.handle_click());
                }
            });
        });
    }

    pub fn handle_click(&self, index: usize, tick: impl Fn() + 'static + Copy) {
        if !self.active.get() { return; }

        if self.interval.with(Option::is_none) {
            self.generate_cells(index);
            self.start_interval(tick);
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
            self.first_clicked_mine_index.set(Some(index));
            set_cell.update(|c| c.mark_as_first_clicked_mine());
            self.click_all_mines();
            self.active.set(false);
            self.face.set(Face::Dead);
            self.clear_interval();
            self.mouse_state.set(MouseState::default());
            self.selected_cell_index.set(None);
            return;
        }

        self.face.set(Face::Happy);

        if cell.is_zero() { self.click_neighboring_empty_cells(index); }
        self.check_for_win();
    }

    pub fn handle_right_click(&self, index: usize) {
        if !self.active.get() { return; }

        let allow_unknown = self.settings.with(|s| s.allow_mark_cell_as_unknown());
        self.grid.with(|g| {
            g[index].update(|c| c.cycle_display(allow_unknown));
        });
        self.face.set(Face::Happy);
    }

    pub fn handle_chord(&self, index: usize, tick: impl Fn() + 'static + Copy) {
        let cell_is_shown = self.grid.with(|g| g[index].with(|c| c.is_shown()));
        if !cell_is_shown {
            self.face.set(Face::Happy);
            return;
        }

        let neighbors = self.neighbors.with(|n| n[index].clone());

        let (neighboring_mines, neighboring_flags) = self.grid.with(|g| {
            let mines = neighbors.iter()
                .filter(|&&idx| g[idx].with(|c| c.is_mine()))
                .count();
            let flags = neighbors.iter()
                .filter(|&&idx| g[idx].with(|c| c.is_flagged()))
                .count();
            (mines, flags)
        });

        if neighboring_mines != neighboring_flags {
            self.face.set(Face::Happy);
            return;
        }

        for neighbor_index in neighbors {
            self.handle_click(neighbor_index, tick);
        }
    }

    pub fn handle_mouse_down(&self, index: usize, event: MouseEvent) {
        if !self.active.get() { return; }

        self.mouse_state.update(|ms| *ms = ms.mouse_down(event));
        self.face.set(Face::Nervous);

        self.mouse_state.with(|mouse_state| {
            match mouse_state {
                MouseState::Left | MouseState::Both => {
                    self.selected_cell_index.set(Some(index));
                },
                MouseState::Right => {
                    self.handle_right_click(index);
                },
                MouseState::AfterBoth | MouseState::Neither => {}
            }
        })
    }

    pub fn handle_mouse_up(&self, index: usize, event: MouseEvent, tick: impl Fn() + 'static + Copy) {
        if !self.active.get() { return; }

        let current_mouse_state = self.mouse_state.get();
        let new_mouse_state = current_mouse_state.mouse_up(event);

        match current_mouse_state {
            MouseState::AfterBoth | MouseState::Neither => {
                self.mouse_state.set(new_mouse_state);
                self.face.set(Face::Happy);
            },
            MouseState::Left => {
                if !new_mouse_state.is_neither() {
                    self.face.set(Face::Happy);
                    return;
                }

                let selected_cell_index = self.selected_cell_index.get();
                if selected_cell_index != Some(index) {
                    self.face.set(Face::Happy);
                    return;
                }

                let chord_setting = self.settings.with(|s| s.chord_setting());
                let cell_is_shown = self.grid.with(|g| g[index].with(|c| c.is_shown()));
                let is_chording = current_mouse_state.is_chording(chord_setting, cell_is_shown);

                self.mouse_state.set(new_mouse_state);

                if is_chording {
                    self.handle_chord(index, tick);
                } else {
                    self.handle_click(index, tick);
                }
            },
            MouseState::Right => {
                self.mouse_state.set(new_mouse_state);
                self.face.set(Face::Happy);
            },
            MouseState::Both => {
                self.handle_chord(index, tick);
                self.mouse_state.set(new_mouse_state);
            }
        }
    }

    fn check_for_win(&self) {
        if self.shown_cells_count() + self.mine_indices.with(Vec::len) == self.grid.with(Vec::len) {
            self.handle_win();
        }
    }

    fn handle_win(&self) {
        self.active.set(false);
        self.face.set(Face::Cool);
        self.flag_all_mines();
        self.clear_interval();
        self.mouse_state.set(MouseState::default());
        self.selected_cell_index.set(None);
    }

    fn flag_all_mines(&self) {
        batch(|| {
            let (mine_inds, g) = (self.mine_indices, self.grid);
            with! { |mine_inds, g|
                for &index in mine_inds.iter() {
                    g[index].update(|cell| cell.set_display_to_flagged());
                }
            }
        });
    }
}
