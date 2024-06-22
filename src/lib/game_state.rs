use std::time::Duration;

use leptos::*;
use leptos::logging::*;
use leptos_dom::helpers::IntervalHandle;

pub fn handle_reset() {}

pub fn clear_interval(interval: WriteSignal<Option<IntervalHandle>>) {
    interval.update(|int|
        *int = match int {
            Some(interval_handle) => {
                interval_handle.clear();
                None::<IntervalHandle>
            },
            None => None::<IntervalHandle>
        }
    );
}

pub fn start_interval(set_interval: WriteSignal<Option<IntervalHandle>>, callback: impl Fn() + 'static) {
    set_interval.update(|i| {
        if i.is_some() {
            log!("Interval already started");
            return;
        }
        *i = match set_interval_with_handle(callback, Duration::from_secs(1)) {
            Ok(interval_handle) => Some(interval_handle),
            Err(e) => {
                error!("Error starting interval: {:?}", e);
                None
            }
        }
    });
}
