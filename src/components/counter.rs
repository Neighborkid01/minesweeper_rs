use leptos::*;

#[component]
pub fn Counter(
    #[prop(into)]
    #[prop(optional)]
    value: Signal<isize>,
) -> impl IntoView {
    view! {
        <div class="counter">
            <span id="timer">{ move || format!("{:0>3}", value()) }</span>
        </div>
    }
}
