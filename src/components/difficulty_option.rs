use yew::prelude::*;
use crate::models::settings::{Difficulty, Dimensions};

#[derive(Properties, PartialEq)]
pub struct DifficultyOptionProps {
    pub on_difficulty_selected: Callback<Difficulty>,
    pub classes: String,
    pub difficulty: Difficulty
}

#[function_component(DifficultyOption)]
pub fn difficulty_option(
    DifficultyOptionProps {classes, difficulty, on_difficulty_selected}: &DifficultyOptionProps
) -> Html {
    html! {
        <a class={classes!("difficulty", classes)}
            onclick={
                // todo!("This only works because custom is hard-coded to have the default dimensions.")
                match difficulty {
                    Difficulty::Beginner => { on_difficulty_selected.reform(|_| Difficulty::Beginner) },
                    Difficulty::Intermediate => { on_difficulty_selected.reform(|_| Difficulty::Intermediate) },
                    Difficulty::Expert => { on_difficulty_selected.reform(|_| Difficulty::Expert) },
                    Difficulty::Custom(_) => { on_difficulty_selected.reform(|_| Difficulty::Custom(Dimensions::default())) },
                }
            }
        >
            {difficulty.title()}
        </a>
    }
}