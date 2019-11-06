use yew::prelude::*;

pub struct ErrorPlaceholder {
    props: ErrorProps,
}

#[derive(Debug, Properties)]
pub struct ErrorProps {
    #[props(required)]
    pub error: Error,
}

#[derive(Debug)]
pub enum Error {
    NotFound,
    NoRunningStegosd,
}

pub enum Messages {}

impl Component for ErrorPlaceholder {
    type Message = Messages;
    type Properties = ErrorProps;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        ErrorPlaceholder { props }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        false
    }
}

impl Renderable<ErrorPlaceholder> for ErrorPlaceholder {
    fn view(&self) -> Html<Self> {
        match self.props.error {
            Error::NotFound => {
                html! {
                <div>
                {"404 - Not found"}
                </div>
                }
            }
            Error::NoRunningStegosd => {
                html! {
                <div>
                {"No running stegosd"}
                </div>
                }
            }
        }
    }
}
