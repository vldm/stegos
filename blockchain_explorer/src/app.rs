use yew::prelude::*;
use yew_router::{route::Route, service::RouteService, Switch};

use crate::block::MacroBlockModel;
use crate::block_list::*;
use crate::placeholder::{Error, ErrorPlaceholder};
use crate::semantic_ui::*;
use log::*;

#[derive(Switch, Debug)]
pub enum Page {
    //    #[to = "/microblock/{epoch}/{offset}"]
    //    MicroBlock { epoch: u64, offset: u32 },
    //    // InvalidBlock
    //    // Transaction
    //    // Output
    //
    //    #[to = "/output/{hash}"]
    //    Output {},
    #[to = "/block/{epoch}"]
    MacroBlock { epoch: u64 },
    #[to = "/{starting_epoch}"]
    BlockList { starting_epoch: u64 },
    #[to = "/"]
    Index,
}

pub struct App {
    route_service: RouteService<()>,
    route: Route<()>,
}

pub enum Msg {
    RouteChanged(Route<()>),
    ChangeRoute(Page),
    BlockList(crate::block_list::Feedback),
    MenuChange(crate::semantic_ui::Feedback),
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, mut link: ComponentLink<Self>) -> Self {
        let mut route_service: RouteService<()> = RouteService::new();
        let route = route_service.get_route();
        let route = Route::from(route);
        let callback = link.send_back(|(route, state)| -> Msg {
            Msg::RouteChanged(Route {
                route,
                state: Some(state),
            })
        });
        route_service.register_callback(callback);
        App {
            route_service,
            route,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::RouteChanged(route) => {
                info!("Route changed!");
                self.route = route
            }
            Msg::ChangeRoute(route) => {
                info!("Change route!");
                // This might be derived in the future
                let route_string = match route {
                    Page::Index => String::from("/"),
                    Page::MacroBlock { epoch } => format!("/block/{}", epoch),
                    Page::BlockList { starting_epoch } => format!("/{}", starting_epoch),
                };
                self.route_service.set_route(&route_string, ());
                self.route = Route {
                    route: route_string,
                    state: None,
                };
            }

            Msg::BlockList(crate::block_list::Feedback::ChangePage { epoch }) => {
                let route_string = format!("/{}", epoch);
                self.route_service.set_route(&route_string, ());
                self.route = Route {
                    route: route_string,
                    state: None,
                };
            }

            Msg::BlockList(crate::block_list::Feedback::ShowBlock { epoch }) => {
                trace!(" SHOW BLOCK = {:?}", epoch);
                let route_string = format!("/block/{}", epoch);
                self.route_service.set_route(&route_string, ());
                self.route = Route {
                    route: route_string,
                    state: None,
                };
            }
            Msg::MenuChange(crate::semantic_ui::Feedback::Click(x)) => match x.as_str() {
                "Main" => {
                    self.route_service.set_route(&"/", ());
                    self.route = Route {
                        route: "/".to_owned(),
                        state: None,
                    };
                }
                s => {
                    self.route_service.set_route(&s, ());
                    self.route = Route {
                        route: s.to_string(),
                        state: None,
                    };
                }
            },
        }
        true
    }
}

impl Renderable<App> for App {
    fn view(&self) -> Html<Self> {
        html! {
            <div class="main ui container">
                <Menu left_items=vec!["Main".to_string()]
                right_items=vec!["3".to_string(), "4".to_string(), "5".to_string()] onchange=Msg::MenuChange />

                <div>
                {
                    match Page::switch(self.route.clone()) {
                        Some(Page::BlockList{ starting_epoch }) => html!{
                            <BlockList onchange=Msg::BlockList starting_epoch={Some(starting_epoch)} />
                        },
                        Some(Page::MacroBlock{ epoch }) => html!{
                            <MacroBlockModel epoch=epoch />
                        },
                        Some(Page::Index) => html!{
                            <BlockList onchange=Msg::BlockList  starting_epoch={None} />
                        },
                        None => html!{
                             <ErrorPlaceholder error= Error::NotFound />
                        }
                    }
                }
                </div>
            </div>
        }
    }
}
