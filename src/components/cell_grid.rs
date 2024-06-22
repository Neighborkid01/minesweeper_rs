use leptos::*;
use web_sys::MouseEvent;
use crate::components::cell_row::CellRow;
use crate::models::{cell::Cell, settings::Settings};

#[component]
pub fn CellGrid(
    grid: ReadSignal<Vec<(ReadSignal<Cell>, WriteSignal<Cell>)>>,
    settings: ReadSignal<Settings>,
    handle_click: impl Fn(MouseEvent, usize) + 'static + Copy,
) -> impl IntoView {
    let rows = move || {
        with! { |grid|
            grid.chunks(settings.with(|s| s.dimensions().width()))
                .enumerate()
                .map(|(i, cells)| {
                    let cells = cells.to_vec();
                    view! {
                        <tr class="game-row">
                            <CellRow
                                row=cells
                                y=i
                                settings
                                handle_click
                            />
                        </tr>
                    }
                })
                .collect_view()
        }
    };

    rows
}
