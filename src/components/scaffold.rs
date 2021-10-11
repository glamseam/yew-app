use yew::prelude::*;
// use yew::services::ConsoleService;
// use yew::{
//     html,
//     Component,
//     ComponentLink,
//     Html,
//     ShouldRender
// };


pub struct Scaffold {}

pub enum Msg {}

impl Component for Scaffold {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <p>{ "Hello world!" }</p>
        }
    }
}
