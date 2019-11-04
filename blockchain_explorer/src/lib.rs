#![recursion_limit = "1024"]
mod app;
pub mod block;
pub mod websocket;
use lazy_static::lazy_static;
use wasm_bindgen::prelude::*;
pub mod semantic_ui;
pub mod temp_api;
use app::App;
pub use temp_api as api;

static TOKEN_FILE: &'static [u8] = include_bytes!(env!("STEGOS_TOKEN"));
const API_TOKENSIZE: usize = 16;

lazy_static! {
    static ref TOKEN: [u8; API_TOKENSIZE] = {
        let token = base64::decode(&TOKEN_FILE).unwrap();
        if token.len() != API_TOKENSIZE {
            panic!("Token size mismatch.")
        }
        let mut token2 = [0u8; API_TOKENSIZE];
        token2.copy_from_slice(&token);
        token2
    };
}

pub static WS_ADDR: &'static str = "ws://localhost:3145";

#[wasm_bindgen]
pub fn run_app() -> Result<(), JsValue> {
    stegos_crypto::set_network_prefix("stt").unwrap();
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));

    yew::start_app::<App>();

    Ok(())
}

#[wasm_bindgen]
pub fn hello_world() {
    use log::trace;
    trace!("Hello World!")
}
