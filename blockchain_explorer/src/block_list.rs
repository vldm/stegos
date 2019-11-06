use yew::prelude::*;
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

#[derive(Clone, Debug)]
pub struct Data {
    inner: MacroBlockHeader,
    hash: String,
}

impl Cells for Data {
    const COUNT: usize = 4;

    fn get(&self, idx: usize) -> &dyn std::fmt::Display {
        match idx {
            0 => &self.inner.epoch,
            1 => &self.hash,
            2 => &self.inner.inputs_len,
            3 => &self.inner.outputs_len,
            _ => unreachable!(),
        }
    }
}

impl WithHeader for Data {
    fn header() -> Vec<&'static dyn std::fmt::Display> {
        vec![&"Epoch", &"Block", &"Inputs", &"Outputs"]
    }
}

pub struct BlockList {
    link: ComponentLink<BlockList>,
    ws: WebSocketSingelton,
    data: Option<TableProperties<Data>>,
    count_of_epochs: Option<u64>,
    props: BlockListProperties,
}
pub enum Feedback {
    ChangePage { epoch: u64 },
    ShowBlock { epoch: u64 },
}
#[derive(Debug, Properties)]
pub struct BlockListProperties {
    pub starting_epoch: Option<u64>,

    #[props(required)]
    pub onchange: Callback<Feedback>,
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
        true
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

        if let Some(pr) = &self.data {
            html! {
            <div class="ui vertical segment">
                <p>{"Blocks history:"}</p>
                <Table<Data> rows=pr.rows.clone() header=pr.header.clone()/>
                <Pagination active_page={page} page_len={total_pages} onchange=BlockListMessage::SwitchTo />
            </div>
            }
        } else {
            html! {

            <div class="ui vertical loading segment">
            <img class="ui wireframe image" src="/img/paragraph.png"/>
            </div>
            }
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
        let rows: Vec<_> = response
            .into_iter()
            .map(|r| {
                let hash = Hash::digest(&r);
                let epoch = r.epoch;
                let row = Row {
                    handler: self
                        .link
                        .send_back(move |_| BlockListMessage::GoToBlock(epoch))
                        .into(),
                    cells: Data {
                        inner: r,
                        hash: hash.to_hex(),
                    },
                };
                row
            })
            .collect();

        self.data = Some(rows.into());
        trace!("draw page = {:?}", self.data);
    }
}
