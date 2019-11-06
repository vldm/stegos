use crate::semantic_ui::{Cells, TableProperties, WithHeader};
use crate::semantic_ui::{Row, Table};
use failure::Error;
use log::*;
use stegos_blockchain::api::MacroBlockInfo;
use stegos_blockchain::{Output, PublicPaymentOutput};
use stegos_crypto::hash::Hash;
use yew::format::Json;
use yew::prelude::*;

use crate::temp_api::{RequestKind, ResponseKind};
use crate::websocket::WebSocketSingelton;
use stegos_node::{ExtendedMacroBlock, NodeRequest, NodeResponse};

#[derive(Clone)]
pub enum State {
    Resolving,
    Resolved { block: ExtendedMacroBlock },
}

pub struct MacroBlockModel {
    ws: WebSocketSingelton,
    state: State,
    props: MacroBlockProperties,
}

#[derive(Debug, Properties)]
pub struct MacroBlockProperties {
    #[props(required)]
    pub epoch: u64,
}

pub enum Messages {
    WsReady(ResponseKind),
}

impl Component for MacroBlockModel {
    type Message = Messages;
    type Properties = MacroBlockProperties;

    fn create(props: Self::Properties, mut link: ComponentLink<Self>) -> Self {
        let cb = link.send_back(Messages::WsReady);
        let mut model = MacroBlockModel {
            ws: WebSocketSingelton::new(cb),
            state: State::Resolving,
            props,
        };
        model.request_utxos();
        model
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Messages::WsReady(m) => self.process_response(m),
        }
        true
    }
}

impl MacroBlockModel {
    fn process_response(&mut self, response: ResponseKind) {
        match response {
            ResponseKind::NodeResponse(NodeResponse::MacroBlockInfo(block)) => {
                debug!("outputs_len = {}", block.block.outputs.len());
                debug!("inputs_len = {}", block.block.inputs.len());
                debug!("block = {:?}", block);
                self.state = State::Resolved { block }
            }
            res => error!("Response didn't expected, response={:?}", res),
        }
    }
    fn request_utxos(&mut self) {
        let request = RequestKind::NodeRequest(NodeRequest::MacroBlockInfo {
            epoch: self.props.epoch,
        });

        self.ws.request(request);
    }
}

impl Renderable<MacroBlockModel> for MacroBlockModel {
    fn view(&self) -> Html<Self> {
        html! {
                <div>
                    {
                        match &self.state
                        {

                         State::Resolved{
                            block
                         }  => {
                          html!{
        //                  rows=self.stake.rows.clone(), header=self.stake.header.clone()
        //                  />
                          <UtxoList utxos=block.block.outputs.clone()/>
                         }
                         },
                         _ =>{
                                html!{}
                            }
                        }
                    }
                </div>
                }
    }
}

#[derive(Default)]
pub struct UtxoList {
    stake: TableProperties<StakeCell>,
    payment: TableProperties<PaymentCell>,
    public: TableProperties<PublicCell>,
}

#[derive(Debug, Properties)]
pub struct UtxoListProperties {
    #[props(required)]
    pub utxos: Vec<Output>,
}

impl Component for UtxoList {
    type Message = ();
    type Properties = UtxoListProperties;

    fn create(props: Self::Properties, mut link: ComponentLink<Self>) -> Self {
        let mut list = UtxoList::default();
        list.set_data_from_utxos(props.utxos);
        list
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.set_data_from_utxos(props.utxos);
        true
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        true
    }
}

impl Renderable<UtxoList> for UtxoList {
    fn view(&self) -> Html<Self> {
        html! {
            <div class="ui vertical segment">
                <p>{"Stake outputs:"}</p>
                <Table<StakeCell> pagination=true rows=self.stake.rows.clone(), header=self.stake.header.clone()/>

                <p>{"Public outputs:"}</p>
                <Table<PublicCell> pagination=true rows=self.public.rows.clone(), header=self.public.header.clone()/>

                <p>{"Payment outputs:"}</p>
                <Table<PaymentCell> pagination=true rows=self.payment.rows.clone(), header=self.payment.header.clone()/>
            </div>
        }
    }
}

impl UtxoList {
    fn set_data_from_utxos(&mut self, outputs: Vec<Output>) {
        let mut stake = Vec::new();
        let mut payment = Vec::new();
        let mut public = Vec::new();

        debug!("outputs_len = {}", outputs.len());
        for output in outputs {
            match output {
                Output::PaymentOutput(p) => payment.push(PaymentCell {
                    hash: Hash::digest(&p).to_hex(),
                    inner: p,
                }),
                Output::PublicPaymentOutput(p) => public.push(PublicCell {
                    hash: Hash::digest(&p).to_hex(),
                    inner: p,
                }),
                Output::StakeOutput(s) => stake.push(StakeCell {
                    hash: Hash::digest(&s).to_hex(),
                    inner: s,
                }),
            };
        }
        self.payment = payment.into();

        self.public = public.into();

        self.stake = stake.into();
    }
}

#[derive(Clone, Debug)]
struct StakeCell {
    inner: stegos_blockchain::StakeOutput,
    hash: String,
}

impl Cells for StakeCell {
    const COUNT: usize = 4;

    fn get(&self, idx: usize) -> &dyn std::fmt::Display {
        match idx {
            0 => &"Unspent",
            1 => &self.hash,
            2 => &self.inner.recipient,
            3 => &self.inner.amount,
            _ => unreachable!(),
        }
    }
}

impl WithHeader for StakeCell {
    fn header() -> Vec<&'static dyn std::fmt::Display> {
        vec![&"Status", &"Hash", &"Recipient", &"Amount"]
    }
}

#[derive(Clone, Debug)]
struct PaymentCell {
    inner: stegos_blockchain::PaymentOutput,
    hash: String,
}

impl Cells for PaymentCell {
    const COUNT: usize = 2;

    fn get(&self, idx: usize) -> &dyn std::fmt::Display {
        match idx {
            0 => &"Unspent",
            1 => &self.hash,
            _ => unreachable!(),
        }
    }
}

impl WithHeader for PaymentCell {
    fn header() -> Vec<&'static dyn std::fmt::Display> {
        vec![&"Status", &"Hash"]
    }
}

#[derive(Clone, Debug)]
struct PublicCell {
    inner: stegos_blockchain::PublicPaymentOutput,
    hash: String,
}

impl Cells for PublicCell {
    const COUNT: usize = 2;

    fn get(&self, idx: usize) -> &dyn std::fmt::Display {
        match idx {
            0 => &"Unspent",
            1 => &self.hash,
            2 => &self.inner.recipient,
            3 => &self.inner.amount,
            _ => unreachable!(),
        }
    }
}

impl WithHeader for PublicCell {
    fn header() -> Vec<&'static dyn std::fmt::Display> {
        vec![&"Status", &"Hash", &"Recipient", &"Amount"]
    }
}
