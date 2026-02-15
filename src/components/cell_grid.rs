use leptos::*;
use crate::components::cell_row::CellRow;
use crate::models::{cell::Cell, settings::Settings};

#[component]
pub fn CellGrid(
    #[prop(into)]
    grid: Signal<Vec<RwSignal<Cell>>>,
    #[prop(into)]
    settings: Signal<Settings>,
) -> impl IntoView {
    move || {
        with! { |grid|
            grid.chunks(settings.with(|s| s.dimensions().width()))
                .enumerate()
                .map(|(i, cells)| {
                    let cells = cells.to_vec();
                    view! {
                        <tr>
                            <CellRow
                                row=cells
                                y=i
                                settings
                            />
                        </tr>
                    }
                })
                .collect_view()
        }
    }
}
