use yew::prelude::*;
// use gloo_console as console;

pub trait Min {
    fn min() -> Self;
}

pub trait Max {
    fn max() -> Self;
}

impl Min for u8 {
    fn min() -> u8 { std::u8::MIN }
}

impl Max for u8 {
    fn max() -> u8 { std::u8::MAX }
}

impl Min for i8 {
    fn min() -> i8 { std::i8::MIN }
}

impl Max for i8 {
    fn max() -> i8 { std::i8::MAX }
}


enum Msg {
    AddOne,
    SubtractOne,
}

struct Model {
    value: i8,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            value: -120,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddOne => {
                if self.value >= <i8 as Max>::max() {
                    self.value = <i8 as Min>::min();
                } else {
                    self.value += 1;
                }
                // the value has changed so we need to
                // re-render for it to appear on the page
                true
            },
            Msg::SubtractOne => {
                if self.value <= <i8 as Min>::min() {
                    self.value = <i8 as Max>::max();
                } else {
                    self.value -= 1;
                }
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // This gives us a component's "`Scope`" which allows us to send messages, etc to the component.
        let link = ctx.link();
        html! {
            <div>
                <button onclick={link.callback(|_| Msg::AddOne)}>{ "+1" }</button>
                <button onclick={link.callback(|_| Msg::SubtractOne)}>{ "-1" }</button>
                <p>{ self.value }</p>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}