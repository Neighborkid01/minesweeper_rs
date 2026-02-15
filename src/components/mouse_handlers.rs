use leptos::*;
use web_sys::MouseEvent;

#[derive(Clone, Copy)]
pub struct MouseHandlers {
    pub handle_mouse_down: StoredValue<Box<dyn Fn(usize, MouseEvent)>>,
    pub handle_mouse_up: StoredValue<Box<dyn Fn(usize, MouseEvent)>>,
    pub handle_mouse_leave: StoredValue<Box<dyn Fn()>>,
}

impl MouseHandlers {
    pub fn new(
        handle_mouse_down: impl Fn(usize, MouseEvent) + 'static,
        handle_mouse_up: impl Fn(usize, MouseEvent) + 'static,
        handle_mouse_leave: impl Fn() + 'static,
    ) -> Self {
        Self {
            handle_mouse_down: store_value(Box::new(handle_mouse_down)),
            handle_mouse_up: store_value(Box::new(handle_mouse_up)),
            handle_mouse_leave: store_value(Box::new(handle_mouse_leave)),
        }
    }
}
