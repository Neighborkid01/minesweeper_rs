use leptos::*;

#[component]
pub fn Counter(
    value: impl Fn() -> isize + 'static,
) -> impl IntoView {
    view! {
        <div class="counter">
            <span id="timer">{ move || format!("{:0>3}", value()) }</span>
        </div>
    }
}
