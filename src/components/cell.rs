use leptos::*;
// use leptos::ev::MouseEvent;
use crate::models::cell::Cell;

#[component]
pub fn Cell(
    index: usize,
    cell: ReadSignal<Cell>,
) -> impl IntoView {
    view! {
        <td key={index} class="cell-border">
            <div
                class=format!("cell {}", with!(|cell| cell.color().to_string()))
            >
                // {move || cell().get_value_display_string()}
                {index}
            </div>
        </td>
    }
}
