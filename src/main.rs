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
    console_error_panic_hook::set_once();

    let game = create_rw_signal(Game::new());
    let tick = move || game.with(|g| g.seconds_played.update(|s| {
        *s = if *s >= 999 { 999 } else { *s + 1 };
    }));

    let settings = Signal::derive(move || game.with(|g| g.settings.get()));
    let on_difficulty_selected = move |difficulty: Difficulty| game.with(|g| g.handle_change_size(difficulty));

    let mines_remaining = move || game.with(|g| g.mines_remaining());
    let reset_game = move |_| game.with(|g| g.handle_reset());
    let face = move || game.with(|g| g.face.get().to_str());
    let seconds_played = move || game.with(|g| g.seconds_played.get());

    let grid = Signal::derive(move || game.with(|g| g.grid.get()));
    let handle_click = move |index: usize| game.with(|g| g.handle_click(index, tick));

    log!("App!");
    view! {
        <div class="container no-select">
            <div class="settings">
                <DifficultyOption
                    difficulty_to_display={Difficulty::Beginner}
                    settings
                    on_difficulty_selected
                />
                <DifficultyOption
                    difficulty_to_display={Difficulty::Intermediate}
                    settings
                    on_difficulty_selected
                />
                <DifficultyOption
                    difficulty_to_display={Difficulty::Expert}
                    settings
                    on_difficulty_selected
                />
                <DifficultyOption
                    difficulty_to_display={Difficulty::Custom(Dimensions::default())}
                    settings
                    on_difficulty_selected
                />
            </div>

            <div class="header">
                <Counter value=mines_remaining />
                <div id="resetButtonContainer" class="center">
                    <span id="resetButton" on:click=reset_game>
                        { face }
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
