use leptos::*;
use crate::models::settings::{Difficulty, Settings};

#[component]
pub fn DifficultyOption(
    on_difficulty_selected: impl Fn(Difficulty) + 'static,
    settings: ReadSignal<Settings>,
    difficulty_to_display: Difficulty,
) -> impl IntoView {
    view! {
        <a class="difficulty"
            class:highlight=move || with!(|settings| settings.difficulty_matches(&difficulty_to_display))
            on:click=move |_| on_difficulty_selected(difficulty_to_display)
        >
            {difficulty_to_display.title()}
        </a>
    }
}
