use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct AppFooterProps {
    #[prop_or_default]
    pub text: String,
}

pub struct AppFooter {
    props: AppFooterProps,
}

impl Component for AppFooter {
    type Message = ();
    type Properties = AppFooterProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        html! {
            <footer>
                {self.props.text.clone()}
            </footer>
        }
    }
}
