mod components;
mod models;

use leptos::*;
use leptos::logging::*;
use components::{
    cell_grid::CellGrid,
    counter::Counter,
    difficulty_option::DifficultyOption,
};
use models::{
    face::Face,
    cell::Cell as Cell,
    mouse_state::MouseState,
    settings::{Difficulty, Settings, Dimensions},
};
use wasm_bindgen::JsCast;
// use yew::{html, Component, Context, Html, classes};
use web_sys::{Element, MouseEvent};
// use gloo_console as console;
use gloo::timers::callback::Interval;
use rand::Rng;
use std::{collections::HashSet, cmp};

#[component]
fn App() -> impl IntoView {
    type CellGrid = Vec<(ReadSignal<Cell>, WriteSignal<Cell>)>;

    let default_difficulty = Difficulty::Beginner;
    let default_grid_area = default_difficulty.dimensions().width()
        * default_difficulty.dimensions().height();
    let grid: CellGrid = (0..default_grid_area)
        .map(|_| create_signal(Cell::new_empty()))
        .collect();
    let mine_indices: Vec<usize> = vec![];
    let neighbors: Vec<HashSet<usize>> = vec![];
    let (active, set_active) = create_signal(false);
    let (face, set_face) = create_signal(Face::default());
    let (grid, set_grid) = create_signal(grid);
    let (neighbors, set_neighbors) = create_signal(neighbors);
    let (mine_indices, set_mine_indices) = create_signal(mine_indices);
    let (shown_cells_count, set_shown_cells_count) = create_signal(0);
    let (selected_cell_index, set_selected_cell_index) = create_signal(None::<usize>);
    let (first_clicked_mine_index, set_first_clicked_mine_index) = create_signal(None::<usize>);
    let (seconds_played, set_seconds_played) = create_signal(0);
    let (mouse_state, set_mouse_state) = create_signal(MouseState::default());
    let (settings, set_settings) = create_signal(Settings::default());
    let (interval, set_interval) = create_signal(None::<Interval>);

    let handle_reset = move || {};

    let handle_change_size = move |difficulty: Difficulty| {
        log!("current size: {:?}", settings().dimensions());
        set_settings.update(|s| { s.set_difficulty(difficulty) });
        log!("changing size to: {:?}", settings().dimensions());
        let dimensions = difficulty.dimensions();
        let grid_area = dimensions.width() * dimensions.height();
        set_grid.set(
            (0..grid_area)
                .map(|_| create_signal(Cell::new_empty()))
                .collect()
        );
        set_neighbors.set(
            (0..grid_area)
                .map(|_| HashSet::new())
                .collect()
        );
        set_mine_indices.set(
            vec![0; dimensions.mines()]
        );
        handle_reset();
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
                    <span id="resetButton" on:click=move |_| handle_reset()>{ move || face().to_str() }</span>
                </div>
                <Counter value=seconds_played />
            </div>

            <div class="board-container">
                <table id="board" class="board"
                    on:contextmenu=move |e: MouseEvent| e.prevent_default()
                >
                    <CellGrid grid settings />
                </table>
            </div>
        </div>
    }
}

fn main() {
    mount_to_body(|| view! { <App /> })
}
