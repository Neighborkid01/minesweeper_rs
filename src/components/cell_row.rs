use leptos::*;
// use web_sys::MouseEvent;
use crate::models::{cell::Cell, settings::Settings};
use crate::components::cell::Cell;

#[component]
pub fn CellRow(
    #[prop(into)]
    row: Vec<(ReadSignal<Cell>, WriteSignal<Cell>)>,
    y: usize,
    settings: ReadSignal<Settings>,
) -> impl IntoView {
    let row_cells = move || {
        row.iter()
            .cloned()
            .enumerate()
            .map(|(x, cell)| {
                let index_offset = y * with!(|settings| settings.dimensions().width());
                (index_offset + x, cell)
            })
            .collect::<Vec<(usize, (ReadSignal<Cell>, WriteSignal<Cell>))>>()
    };

    view! {
        <For
            each=row_cells
            key=|tuple| tuple.0
            children=move |(index, (cell, _set_cell))| {
                view! {
                    <Cell index cell />
                }
            }
        />
    }
}
