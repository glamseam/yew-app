use yew::prelude::*;
use yew::services::ConsoleService;

use js_sys::Date;

pub enum Msg {
    Increment,
    Decrement,
}

pub struct IncDec {
    link: ComponentLink<Self>,
    value: i64,
}

impl Component for IncDec {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, value: 0 }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Increment => {
                self.value += 1;
                ConsoleService::log("plus one");
                true
            }
            Msg::Decrement => {
                self.value -= 1;
                ConsoleService::log("minus one");
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <nav class="menu">
                    <button onclick=self.link.callback(|_| Msg::Increment)>
                        { "Increment" }
                    </button>
                    <button onclick=self.link.callback(|_| Msg::Decrement)>
                        { "Decrement" }
                    </button>
                    <button onclick=self.link.batch_callback(|_| vec![Msg::Increment, Msg::Increment])>
                        { "Increment Twice" }
                    </button>
                </nav>
                <p>
                    <b>{ "Current value: " }</b>
                    { self.value }
                </p>
                <p>
                    <b>{ "Rendered at: " }</b>
                    { String::from(Date::new_0().to_string()) }
                </p>
            </div>
        }
    }
}
