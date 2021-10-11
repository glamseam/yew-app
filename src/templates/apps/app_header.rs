use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct AppHeaderProps {
    #[prop_or("Palyground".to_string())]
    pub text: String,
}

pub struct AppHeader {
    props: AppHeaderProps,
}

impl Component for AppHeader {
    type Message = ();
    type Properties = AppHeaderProps;

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
            <header>
                {self.props.text.clone()}
            </header>
        }
    }
}
