mod components;
mod models;

use leptos::*;
use leptos::logging::*;
use components::counter::Counter;
use components::difficulty_option::DifficultyOption;
use models::face::Face;
use models::cell::Cell as Cell;
use models::mouse_state::MouseState;
use models::settings::{Difficulty, Settings, Dimensions};
use wasm_bindgen::JsCast;
// use yew::{html, Component, Context, Html, classes};
use web_sys::{Element, MouseEvent};
// use gloo_console as console;
use gloo::timers::callback::Interval;
use rand::Rng;
use std::{collections::HashSet, cmp};

#[component]
fn App() -> impl IntoView {
    log!("App!");
}

fn main() {
    mount_to_body(|| view! { <App /> })
}
