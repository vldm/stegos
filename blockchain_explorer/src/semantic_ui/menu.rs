use log::*;
use yew::prelude::*;

pub type Item = String;

#[derive(Debug)]
pub struct Menu {
    props: Props,
}

#[derive(Debug, Properties)]
pub struct Props {
    pub left_items: Vec<Item>,
    pub right_items: Vec<Item>,
    #[props(required)]
    pub onchange: Callback<Feedback>,
}

#[derive(Debug)]
pub enum Feedback {
    Click(Item),
}

pub enum Msg {
    ClickLeft(usize),
    ClickRight(usize),
}

impl Component for Menu {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Menu { props }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ClickLeft(idx) => {
                trace!("Click left = {}", idx);
                let item = self.props.left_items.get(idx).expect("Exist item").clone();
                self.props.onchange.emit(Feedback::Click(item));
            }

            Msg::ClickRight(idx) => {
                trace!("Click right = {}", idx);
                let item = self.props.right_items.get(idx).expect("Exist item").clone();
                self.props.onchange.emit(Feedback::Click(item));
            }
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }
}

impl Renderable<Menu> for Menu {
    fn view(&self) -> Html<Self> {
        html! {
        <div class="ui attached stackable menu">
            {
            for self.props.left_items.iter().enumerate().map(|(idx, name)|{
                html!{
                    <a class="item" role="button" onclick=|_|  Msg::ClickLeft(idx)>
                        {name}
                    </a>
                }
            })
            }

            {
            if !self.props.right_items.is_empty() {
                html!{
                <div class="right menu">
                    {
                    for self.props.right_items.iter().enumerate().map(|(idx, name)|{
                        html!{
                            <a class="item" role="button" onclick=|_|  Msg::ClickRight(idx)>
                                {name}
                            </a>
                        }
                    })
                    }
                </div>
                }
            }
            else {
            html!{}
            }
            }
        </div>
        }
    }
}
