use leptos::*;
use leptos::ev::MouseEvent;
use crate::models::cell::Cell;

#[component]
pub fn Cell(
    index: usize,
    cell: (ReadSignal<Cell>, WriteSignal<Cell>),
    handle_click: impl Fn(MouseEvent, usize) + 'static + Copy,
) -> impl IntoView {
    let (cell, set_cell) = cell;

    view! {
        <td key={index} class="cell-border">
            <div
                class=move || format!("cell {}", cell.with(|c| c.color().to_string()))
                on:click=move |e| {
                    handle_click(e, index);
                    set_cell.update(|cell| cell.handle_click());
                }
            >
                // {move || cell().get_value_display_string()}
                {index}
            </div>
        </td>
    }
}
