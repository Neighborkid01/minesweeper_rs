use leptos::*;
// use leptos::ev::MouseEvent;
use crate::models::cell::Cell;

#[component]
pub fn Cell(
    index: usize,
    cell: RwSignal<Cell>,
    handle_click: impl Fn(usize) + 'static + Copy,
) -> impl IntoView {

    view! {
        <td key={index} class="cell-border">
            <div
                class=move || cell.with(|c| format!("cell {}", c.color()))
                class:clicked=move || cell.with(|c| c.is_shown())
                class:mine=move || cell.with(|c| c.is_first_clicked_mine())
                on:click=move |_e| {
                    handle_click(index);
                    // set_cell.update(|cell| cell.handle_click());
                }
            >
                {move || cell.with(|c| c.value_display_string())}
            </div>
        </td>
    }
}
