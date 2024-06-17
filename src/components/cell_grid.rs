use leptos::*;
use leptos::logging::*;
// use web_sys::MouseEvent;
use crate::components::cell_row::CellRow;
use crate::models::{cell::Cell, settings::Settings};

#[component]
pub fn CellGrid(
    grid: ReadSignal<Vec<(ReadSignal<Cell>, WriteSignal<Cell>)>>,
    settings: ReadSignal<Settings>,
) -> impl IntoView {
    let rows = move || {
        with! { |grid, settings|
            grid.chunks(settings.dimensions().width())
                .enumerate()
                .map(|(i, cells)| {
                    let cells = cells.to_vec();
                    let len = cells.len();
                    // Ok this makes no sense. if len is in the key it works otherwise it doesn't...
                    let key = format!("{}-{}-{}-{}", len, settings.dimensions().width(), settings.dimensions().height(), i);
                    (key, i, len, cells)
                })
                .collect::<Vec<(String, usize, usize, Vec<(ReadSignal<Cell>, WriteSignal<Cell>)>)>>()
        }
    };

    view! {
        <For
            each=rows
            key=|tuple| tuple.0.clone()
            children=move |(key, y, len, row)| {
                view! {
                    <tr key={key} id={len} class="game-row">
                        <div>{format!("{}", row.len())}</div>
                        <CellRow row y settings />
                    </tr>
                }
            }
        />
    }
}
