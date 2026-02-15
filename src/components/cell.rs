use leptos::*;
use leptos::ev::MouseEvent;
use leptos::html::Div;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use std::rc::Rc;
use std::cell::RefCell;
use crate::models::cell::Cell;
use crate::components::mouse_handlers::MouseHandlers;

type MouseLeaveListener = Rc<RefCell<Option<Closure<dyn Fn(web_sys::MouseEvent)>>>>;

#[component]
pub fn Cell(
    index: usize,
    cell: RwSignal<Cell>,
) -> impl IntoView {
    let mouse_handlers = use_context::<MouseHandlers>()
        .expect("MouseHandlers context not found");

    let div_ref: NodeRef<Div> = create_node_ref();

    let listener_state: MouseLeaveListener = Rc::new(RefCell::new(None));
    let listener_state_clone = Rc::clone(&listener_state);

    let add_listener = Rc::new(move || {
        // Only add if we don't already have one
        if listener_state_clone.borrow().is_some() {
            return;
        }

        let Some(element) = div_ref.get() else {
            return;
        };

        // Clone for the inner closure that removes itself
        let listener_state_inner = Rc::clone(&listener_state_clone);

        let closure = Closure::wrap(Box::new(move |_e: web_sys::MouseEvent| {
            mouse_handlers.handle_mouse_leave.with_value(|f| f());

            if let Some(closure) = listener_state_inner.borrow_mut().take() {
                if let Some(el) = div_ref.get_untracked() {
                    let _ = el.remove_event_listener_with_callback(
                        "mouseleave",
                        closure.as_ref().unchecked_ref()
                    );
                }
            }
        }) as Box<dyn Fn(web_sys::MouseEvent)>);

        let _ = element.add_event_listener_with_callback(
            "mouseleave",
            closure.as_ref().unchecked_ref()
        );

        *listener_state_clone.borrow_mut() = Some(closure);
    });

    let remove_listener = Rc::new(move || {
        if let Some(closure) = listener_state.borrow_mut().take() {
            if let Some(element) = div_ref.get() {
                let _ = element.remove_event_listener_with_callback(
                    "mouseleave",
                    closure.as_ref().unchecked_ref()
                );
            }
        }
    });

    view! {
        <td key={index} class="cell-border">
            <div
                node_ref=div_ref
                class=move || {
                    let mut classes = "cell".to_string();
                    cell.with(|c| {
                        classes.push_str(&format!(" {}", c.color()));
                        if c.is_shown() || c.is_highlighted() {
                            classes.push_str(" clicked");
                        }
                        if c.is_first_clicked_mine() {
                            classes.push_str(" mine");
                        }
                    });
                    classes
                }
                on:mousedown=move |e: MouseEvent| {
                    e.prevent_default();
                    add_listener();
                    mouse_handlers.handle_mouse_down.with_value(|f| f(index, e));
                }
                on:mouseup=move |e: MouseEvent| {
                    e.prevent_default();
                    remove_listener();
                    mouse_handlers.handle_mouse_up.with_value(|f| f(index, e));
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
