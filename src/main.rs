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
    cell::Cell as Cell,
    game::*,
    settings::{Difficulty, Dimensions},
};
use web_sys::{MouseEvent};

type CellGrid = Vec<RwSignal<Cell>>;

#[component]
fn App() -> impl IntoView {
    console_error_panic_hook::set_once();

    let game = create_rw_signal(Game::new());

    let settings = Signal::derive(move || game.with(|g| g.settings.get()));
    let on_difficulty_selected = move |difficulty: Difficulty| game.with(|g| g.handle_change_size(difficulty));

    let mines_remaining = move || game.with(|g| g.mines_remaining());
    let reset_game = move |_| game.with(|g| g.handle_reset());
    let face = move || game.with(|g| g.face.get().as_str());
    let seconds_played = move || game.with(|g| g.seconds_played.get());

    let grid = Signal::derive(move || game.with(|g| g.grid.get()));
    let handle_mouse_down = move |index: usize, event: MouseEvent| game.with(|g| g.handle_mouse_down(index, event));
    let handle_mouse_up = move |index: usize, event: MouseEvent| game.with(|g| g.handle_mouse_up(index, event));

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
                        handle_mouse_down
                        handle_mouse_up
                    />
                </table>
            </div>
        </div>
    }
}

fn main() {
    mount_to_body(|| view! { <App /> })
}
