use yew::prelude::*;

pub type Page = usize;

#[derive(Debug)]
pub struct Pagination {
    props: Props,
}

#[derive(Debug, Properties)]
pub struct Props {
    pub page_len: Option<Page>,
    pub active_page: Page,
    #[props(required)]
    pub onchange: Callback<Page>,
}

pub enum Msg {
    Click(Page),
    Next,
    Previous,
}

impl Component for Pagination {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Pagination { props }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Click(value) => {
                self.props.active_page = value;
                self.props.onchange.emit(self.props.active_page);
            }
            //TODO: Add bound checks
            Msg::Next => {
                self.props.active_page += 1;
                self.props.onchange.emit(self.props.active_page);
            }
            Msg::Previous => {
                self.props.active_page = self.props.active_page.saturating_sub(1);
                self.props.onchange.emit(self.props.active_page);
            }
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }
}

impl Renderable<Pagination> for Pagination {
    fn view(&self) -> Html<Self> {
        html! {
            <div class="ui pagination menu">
                <a class="item" role="button" onclick=|_|  Msg::Previous>
                    <i class="angle left icon"></i>
                </a>
                {
                    for (0..(self.props.page_len.unwrap_or(10))).map(|idx|{
                        let is_active = self.props.active_page == idx;
                        let mut class = "item";
                        if is_active {
                            class = "item active"
                        }
                        html! {
                            <a class={class} role="button" onclick=|_|  Msg::Click(idx)>{idx+1}</a>
                        }
                    })
                }
                <a class="item" role="button" onclick=|_|  Msg::Next>
                    <i class="angle right icon"></i>
                </a>
            </div>
        }
    }
}
