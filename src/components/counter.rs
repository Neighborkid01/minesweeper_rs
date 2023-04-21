use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct CounterProps {
  pub value: isize,
  pub classes: String
}

#[function_component(Counter)]
pub fn counter(CounterProps { value, classes }: &CounterProps) -> Html {
  html! {
    <div class={classes!("counter", classes)}>
      <span id="timer">{ format!("{:0>3}", value) }</span>
    </div>
  }
}