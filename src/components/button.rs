use yew::prelude::*;

#[derive(Clone, Properties, PartialEq)]
pub struct ButtonProps {
    pub label: String,
    #[prop_or_else(Callback::noop)]
    pub onclick: Callback<MouseEvent>,
}

pub struct Button {
    props: ButtonProps,
}

impl Component for Button {
    type Message = ();
    type Properties = ButtonProps;

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
            <button
                class="button-class"
                type="button"
                onclick=self.props.onclick.clone()
            >
                {self.props.label.clone()}
            </button>
        }
    }
}
