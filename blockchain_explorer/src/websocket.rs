use crate::api;
use failure::Error;
use log::*;
use serde::{de::DeserializeOwned, Serialize};
use wasm_bindgen::__rt::core::time::Duration;
use wasm_bindgen::__rt::std::collections::HashMap;
use yew::callback::Callback;
use yew::services::timeout::TimeoutTask;
use yew::services::websocket::{WebSocketStatus, WebSocketTask};
use yew::services::{TimeoutService, WebSocketService};
use yew::worker::*;

pub struct WebSocketSingelton {
    context: Box<dyn Bridge<WebSocketAgent>>,
}

impl WebSocketSingelton {
    pub fn new(callback: Callback<crate::api::ResponseKind>) -> Self {
        let context = WebSocketAgent::bridge(callback);
        WebSocketSingelton { context }
    }

    pub fn request(&mut self, kind: crate::api::RequestKind) {
        self.context.send(kind)
    }
}

struct WebSocketAgent {
    ws_service: WebSocketService,
    ws_task: Option<WebSocketTask>,

    timeout_service: TimeoutService,
    timeout: Option<TimeoutTask>,

    requests: HashMap<u64, HandlerId>,
    pending_request: Vec<(crate::api::RequestKind, HandlerId)>,
    last_request: u64,
    connected: bool,
    link: AgentLink<WebSocketAgent>,
}

enum Msg {
    Connected,
    Read(Result<String, Error>),
    Lost,
    Reconnect,
}

impl Agent for WebSocketAgent {
    // Available:
    // - `Job` (one per bridge on the main thread)
    // - `Context` (shared in the main thread)
    // - `Private` (one per bridge in a separate thread)
    // - `Public` (shared in a separate thread)
    type Reach = Context;
    type Message = Msg;
    type Input = crate::api::RequestKind;
    type Output = crate::api::ResponseKind;

    // Create an instance with a link to agent's environment.
    fn create(link: AgentLink<Self>) -> Self {
        let mut socket = WebSocketAgent {
            ws_service: WebSocketService::new(),
            timeout_service: TimeoutService::new(),
            link,
            ws_task: None,
            timeout: None,
            pending_request: vec![],
            requests: HashMap::new(),
            last_request: 0,
            connected: false,
        };
        socket.connect();
        socket
    }

    fn update(&mut self, msg: Self::Message) {
        match msg {
            Msg::Lost => {
                trace!("Websocket failed, reconnecting in 10 secs...");
                self.ws_task = None;
                let send_msg = self.link.send_back(|_| Msg::Reconnect.into());
                self.timeout = Some(
                    self.timeout_service
                        .spawn(Duration::from_secs(10), send_msg),
                );
            }
            Msg::Reconnect => {
                self.connected = false;
                self.connect();
                self.timeout = None;
            }
            Msg::Connected => {
                self.connected = true;
                self.process_queue();
            }
            Msg::Read(response) => {
                let response =
                    response.and_then(|a| crate::api::decode::<crate::api::Response>(&a));
                match response {
                    Ok(response) => {
                        let id = response.id;
                        let kind = response.kind;
                        let who = self.requests.get(&id);
                        let who = if let Some(who) = who {
                            who
                        } else {
                            error!("Pending request with that id not found");
                            return;
                        };
                        self.link.response(*who, kind)
                    }
                    Err(e) => error!("Error during processing response ={}", e),
                }
            }
        }
    }

    // Handle incoming messages from components of other agents.
    fn handle(&mut self, msg: Self::Input, who: HandlerId) {
        trace!("received request");
        if self.connected {
            self.request(msg, who)
        } else {
            self.pending_request.push((msg, who))
        }
    }
}

pub struct RawResponse(Result<String, Error>);
impl From<Result<String, Error>> for RawResponse {
    fn from(d: Result<String, Error>) -> Self {
        RawResponse(d)
    }
}
impl From<Result<Vec<u8>, Error>> for RawResponse {
    fn from(d: Result<Vec<u8>, Error>) -> Self {
        RawResponse(d.and_then(|s| String::from_utf8(s).map_err(Into::into)))
    }
}

impl WebSocketAgent {
    fn connect(&mut self) {
        if let Some(_) = self.ws_task {
            warn!("Websocket connection already active");
        } else {
            let callback = self.link.send_back(|RawResponse(data)| Msg::Read(data));
            let notification = self.link.send_back(|status| match status {
                WebSocketStatus::Opened => Msg::Connected.into(),
                WebSocketStatus::Closed | WebSocketStatus::Error => Msg::Lost.into(),
            });
            let task = self
                .ws_service
                .connect(crate::WS_ADDR, callback, notification);
            self.ws_task = Some(task);
        }
    }

    fn get_request_id(&mut self) -> u64 {
        let id = self.last_request + 1;
        self.last_request = id;
        id
    }
    pub fn process_queue(&mut self) {
        let requests = std::mem::replace(&mut self.pending_request, vec![]);
        trace!("Connected to ws, processing queue = {}", requests.len());
        for (kind, who) in requests {
            self.request(kind, who)
        }
    }

    fn request(&mut self, kind: crate::api::RequestKind, who: HandlerId) {
        let mut id = self.get_request_id();
        let ws = match &mut self.ws_task {
            Some(ws) => ws,
            None => {
                panic!("Can't send message to closed websocket.");
            }
        };

        let request = crate::api::Request { id, kind };

        assert!(self.requests.insert(id, who).is_none());

        let request = crate::api::encode(&request);
        ws.send(Ok(request))
    }
}
