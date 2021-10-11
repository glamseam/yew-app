use yew::prelude::*;

use crate::components::button::Button;
use crate::components::count::Count;
use crate::components::inc_dec::IncDec;
use crate::templates::apps::app_footer::AppFooter;
use crate::templates::apps::app_header::AppHeader;

pub struct App {}

impl Component for App {
    type Message = ();
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
            <div id="app">
                <AppHeader />
                <Button label="ボタン" />
                <Count />
                <IncDec />
                <AppFooter text="Footer" />
            </div>
        }
    }
}
