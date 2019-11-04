use yew::prelude::*;
use yew_router::{route::Route, service::RouteService, Switch};

use crate::block::MacroBlockModel;
use crate::semantic_ui::*;
use crate::temp_api::{RequestKind, ResponseKind};
use crate::websocket::WebSocketSingelton;
use log::*;
use stegos_blockchain::{ElectionInfo, MacroBlockHeader};
use stegos_crypto::hash::Hash;
use stegos_node::{NodeRequest, NodeResponse};

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
    BlockList(Feedback),
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
            Msg::RouteChanged(route) => self.route = route,
            Msg::ChangeRoute(route) => {
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

            Msg::BlockList(Feedback::ChangePage { epoch }) => {
                let route_string = format!("/{}", epoch);
                self.route_service.set_route(&route_string, ());
                self.route = Route {
                    route: route_string,
                    state: None,
                };
            }

            Msg::BlockList(Feedback::ShowBlock { epoch }) => {
                trace!(" SHOW BLOCK = {:?}", epoch);
                let route_string = format!("/block/{}", epoch);
                self.route_service.set_route(&route_string, ());
                self.route = Route {
                    route: route_string,
                    state: None,
                };
            }
            Msg::MenuChange(x) => error!("x = {:?}", x),
        }
        true
    }
}

pub const ACTIVE_CLASS: &'static str = "active item";

impl Renderable<App> for App {
    fn view(&self) -> Html<Self> {
        html! {
            <div>
                <nav class="menu",>
                    <button onclick=|_|  Msg::ChangeRoute(Page::BlockList{ starting_epoch:1 }) > {"1"} </button>
                    <button onclick=|_|  Msg::ChangeRoute(Page::BlockList{ starting_epoch:2 }) > {"2"} </button>
                    <button onclick=|_|  Msg::ChangeRoute(Page::Index) > {"Main"} </button>
                </nav>
                <Menu left_items=vec!["1".to_string(), "2".to_string(), "3".to_string()]
                right_items=vec!["3".to_string(), "4".to_string(), "5".to_string()] onchange=Msg::MenuChange />

                <div>
                {
                    match Page::switch(self.route.clone()) {
                        Some(Page::BlockList{ starting_epoch }) => html!{
                            <BlockList onchange=Msg::BlockList starting_epoch={Some(starting_epoch)} />
                        },
                        Some(Page::MacroBlock{ epoch }) => html!{
                            <MacroBlockModel  />
                        },
                        Some(Page::Index) => html!{
                            <BlockList onchange=Msg::BlockList  starting_epoch={None} />
                        },
                        None => html!{
                             {format!("404")}
                        }
                    }
                }
                </div>
            </div>
        }
    }
}

pub struct BlockList {
    link: ComponentLink<BlockList>,
    ws: WebSocketSingelton,
    data: Option<TableProperties>,
    count_of_epochs: Option<u64>,
    props: BlockListProperties,
}
pub enum Feedback {
    ChangePage { epoch: u64 },
    ShowBlock { epoch: u64 },
}
#[derive(Debug, Properties)]
pub struct BlockListProperties {
    starting_epoch: Option<u64>,

    #[props(required)]
    onchange: Callback<Feedback>,
}

pub enum BlockListMessage {
    WsReady(ResponseKind),
    SwitchTo(usize),
    GoToBlock(u64),
    Ignore,
}

impl Component for BlockList {
    type Message = BlockListMessage;
    type Properties = BlockListProperties;

    fn create(props: Self::Properties, mut link: ComponentLink<Self>) -> Self {
        let cb = link.send_back(BlockListMessage::WsReady);
        let mut list = BlockList {
            link,
            ws: WebSocketSingelton::new(cb),
            data: None,
            props,
            count_of_epochs: None,
        };

        list.request_block_count();
        list
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        //        list.request_block_count();
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            BlockListMessage::Ignore => return false,
            BlockListMessage::WsReady(response) => self.match_response(response),
            BlockListMessage::SwitchTo(x) => {
                let starting_epoch =
                    if self.count_of_epochs.unwrap_or(0) > x as u64 * BLOCKS_PER_PAGE {
                        self.count_of_epochs.unwrap_or(0) - x as u64 * BLOCKS_PER_PAGE
                    } else {
                        0
                    };
                self.props.starting_epoch = Some(starting_epoch);
                self.props.onchange.emit(Feedback::ChangePage {
                    epoch: starting_epoch,
                });
                self.request_blocks();
                error!("starting_epoch = {}", starting_epoch)
            }
            BlockListMessage::GoToBlock(b) => {
                self.props.onchange.emit(Feedback::ShowBlock { epoch: b });
            }
        }
        true
    }
}

impl Renderable<BlockList> for BlockList {
    fn view(&self) -> Html<Self> {
        let total_pages =
            Some(u64::min(self.count_of_epochs.unwrap_or(0) / BLOCKS_PER_PAGE, 20) as usize);
        let page = self
            .count_of_epochs
            .map(|epochs| epochs - self.props.starting_epoch.unwrap_or(0))
            .unwrap_or(0) as usize
            / BLOCKS_PER_PAGE as usize;
        info!("active_page = {}", page);

        html! {
            <div>
            {
            if let Some(pr) = &self.data {
                html!{
                <Table rows=pr.rows.clone() header=pr.header.clone()/>
                }
            }
            else {html!{}}
            }
            <Pagination active_page={page} page_len={total_pages} onchange=BlockListMessage::SwitchTo />
            </div>
        }
    }
}

const BLOCKS_PER_PAGE: u64 = 100;

#[derive(Copy, Clone)]
pub struct RequestId;

impl BlockList {
    fn request_block_count(&mut self) {
        let request = RequestKind::NodeRequest(NodeRequest::ElectionInfo {});
        self.ws.request(request);
    }

    fn request_blocks(&mut self) {
        let request = RequestKind::NodeRequest(NodeRequest::BlockList {
            epoch: self.props.starting_epoch.unwrap(),
            limit: BLOCKS_PER_PAGE as u64,
        });

        self.ws.request(request);
    }

    fn match_response(&mut self, response: ResponseKind) {
        match response {
            ResponseKind::NodeResponse(NodeResponse::ElectionInfo(e)) => self.init_count(e),
            ResponseKind::NodeResponse(NodeResponse::BlockList { list }) => self.draw_page(list),
            res => error!("Response didn't expected, response={:?}", res),
        }
    }

    fn init_count(&mut self, response: ElectionInfo) {
        let epoch = response.epoch - 1; // epoch not ended
        info!("Set epoch count to = {}", epoch);
        self.count_of_epochs = Some(epoch);
        self.props.starting_epoch = self.props.starting_epoch.unwrap_or(epoch).into();
        self.props.onchange.emit(Feedback::ChangePage { epoch });
        self.request_blocks();
    }

    fn draw_page(&mut self, response: Vec<MacroBlockHeader>) {
        let header: Row = vec![
            "Epoch".to_string(),
            "Block".to_string(),
            "Inputs".to_string(),
            "Outputs".to_string(),
        ]
        .into();
        let rows: Vec<Row> = response
            .into_iter()
            .map(|r| {
                let epoch = r.epoch;
                let hash = Hash::digest(&r);
                let inputs = r.inputs_len;
                let outputs = r.outputs_len;
                let row = Row {
                    cells: vec![
                        epoch.to_string().into(),
                        hash.to_string().into(),
                        inputs.to_string().into(),
                        outputs.to_string().into(),
                    ]
                    .into(),
                    handler: self
                        .link
                        .send_back(move |_| BlockListMessage::GoToBlock(epoch))
                        .into(),
                };
                row
            })
            .collect();

        self.data = Some(TableProperties {
            rows,
            header: Some(header),
        });
        trace!("draw page = {:?}", self.data);
    }
}
