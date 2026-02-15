use leptos::*;
use crate::models::{cell::Cell, settings::Settings};
use crate::components::cell::Cell;

#[component]
pub fn CellRow(
    row: Vec<RwSignal<Cell>>,
    y: usize,
    #[prop(into)]
    settings: Signal<Settings>,
) -> impl IntoView {
    let row_cells = move || {
        row.iter()
            .enumerate()
            .map(|(x, cell)| {
                let index = x + (y * settings.with(|s| s.dimensions().width()));
                view! {
                    <Cell index cell={*cell} />
                }
            })
            .collect_view()
    };

    row_cells
}
