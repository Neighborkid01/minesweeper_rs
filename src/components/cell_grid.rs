use leptos::*;
use leptos::ev::MouseEvent;
use crate::components::cell_row::CellRow;
use crate::models::{cell::Cell, settings::Settings};

#[component]
pub fn CellGrid(
    #[prop(into)]
    grid: Signal<Vec<RwSignal<Cell>>>,
    #[prop(into)]
    settings: Signal<Settings>,
    handle_mouse_down: impl Fn(usize, MouseEvent) + 'static + Copy,
    handle_mouse_up: impl Fn(usize, MouseEvent) + 'static + Copy,
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
                                handle_mouse_down
                                handle_mouse_up
                            />
                        </tr>
                    }
                })
                .collect_view()
        }
    }
}
