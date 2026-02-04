use leptos::*;
use leptos::ev::MouseEvent;
use crate::models::cell::Cell;

#[component]
pub fn Cell(
    index: usize,
    cell: RwSignal<Cell>,
    handle_mouse_down: impl Fn(usize, MouseEvent) + 'static + Copy,
    handle_mouse_up: impl Fn(usize, MouseEvent) + 'static + Copy,
) -> impl IntoView {

    view! {
        <td key={index} class="cell-border">
            <div
                class=move || cell.with(|c| format!("cell {}", c.color()))
                class:clicked=move || cell.with(|c| c.is_shown())
                class:mine=move || cell.with(|c| c.is_first_clicked_mine())
                on:mousedown=move |e: MouseEvent| {
                    e.prevent_default();
                    handle_mouse_down(index, e);
                }
                on:mouseup=move |e: MouseEvent| {
                    e.prevent_default();
                    handle_mouse_up(index, e);
                }
                on:contextmenu=move |e: MouseEvent| {
                    e.prevent_default();
                }
            >
                {move || cell.with(|c| c.value_display_string())}
            </div>
        </td>
    }
}
