use leptos::*;
use web_sys::MouseEvent;
use crate::models::{cell::Cell, settings::Settings};
use crate::components::cell::Cell;

#[component]
pub fn CellRow(
    row: Vec<(ReadSignal<Cell>, WriteSignal<Cell>)>,
    y: usize,
    settings: ReadSignal<Settings>,
    handle_click: impl Fn(MouseEvent, usize) + 'static + Copy,
) -> impl IntoView {
    let row_cells = move || {
        row.iter()
            .cloned()
            .enumerate()
            .map(|(x, cell)| {
                let index = x + (y * settings.with(|s| s.dimensions().width()));
                view! {
                    <Cell
                        index
                        cell
                        handle_click
                    />
                }
            })
            .collect_view()
    };

    row_cells
}
