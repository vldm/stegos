use serde::de::DeserializeOwned;

use aes_ctr::{
    stream_cipher::{NewStreamCipher, SyncStreamCipher},
    Aes128Ctr,
};

use failure::{bail, Error};
use rand::{rngs::OsRng, RngCore};
use serde_derive::{Deserialize, Serialize};
use stegos_crypto::pbc;
use stegos_node::{
    api::{NodeRequest, NodeResponse},
    ChainNotification, StatusNotification,
};

#[derive(Debug, Clone, Copy)]
struct ApiToken(pub(crate) &'static [u8]);
use std::iter::repeat;

// Encrypts the plaintext with given 32-byte key
// returns encrypted payload with 16-byte IV prepended
pub fn encrypt(plaintext: &[u8]) -> Vec<u8> {
    let mut nonce: Vec<u8> = vec![0; 16];

    let mut gen = OsRng;
    gen.fill_bytes(&mut nonce[..]);
    let mut aes_enc = Aes128Ctr::new_var(&*crate::TOKEN, &nonce).unwrap();

    let mut output: Vec<u8> = repeat(0u8).take(16 + plaintext.len()).collect();
    output[..16].copy_from_slice(&nonce[..]);
    output[16..].copy_from_slice(plaintext);
    aes_enc.apply_keystream(&mut output[16..]);
    output
}

pub fn decrypt(ciphertext: &[u8]) -> Vec<u8> {
    let mut iv: Vec<u8> = repeat(0u8).take(16).collect();
    iv[..].copy_from_slice(&ciphertext[..16]);
    let mut aes_enc = Aes128Ctr::new_var(&*crate::TOKEN, &iv).unwrap();
    let mut output: Vec<u8> = ciphertext[16..].to_vec();
    aes_enc.apply_keystream(&mut output);
    output
}

pub fn encode<T: serde::Serialize>(msg: &T) -> String {
    let msg = serde_json::to_vec(&msg).expect("serialized");
    let msg = encrypt(&msg);
    let msg = base64::encode(&msg);
    msg
}

pub fn decode<T: DeserializeOwned>(msg: &str) -> Result<T, Error> {
    let msg = match base64::decode(&msg) {
        Ok(r) => r,
        Err(e) => {
            bail!("Failed to base64::decode ={}", e);
        }
    };
    let msg = decrypt(&msg);
    // Check for {} brackets in decoded message.
    const LEFT_BRACKET: u8 = 123;
    const RIGHT_BRACKET: u8 = 125;
    if msg.len() < 2 || msg[0] != LEFT_BRACKET || msg[msg.len() - 1] != RIGHT_BRACKET {
        bail!("Failed to decrypt")
    }

    let msg: T = match serde_json::from_slice(&msg) {
        Ok(r) => r,
        Err(e) => {
            bail!("Failed to parse JSON ={}", e);
        }
    };
    Ok(msg)
}

pub type RequestId = u64;

fn is_request_id_default(id: &RequestId) -> bool {
    *id == 0
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum NetworkRequest {
    // VersionInfo is not about Network, but let's keep it here to simplify all things.
    VersionInfo {},
    SubscribeUnicast {
        topic: String,
    },
    SubscribeBroadcast {
        topic: String,
    },
    UnsubscribeUnicast {
        topic: String,
    },
    UnsubscribeBroadcast {
        topic: String,
    },
    SendUnicast {
        topic: String,
        to: pbc::PublicKey,
        data: Vec<u8>,
    },
    PublishBroadcast {
        topic: String,
        data: Vec<u8>,
    },
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum NetworkResponse {
    VersionInfo { version: String },
    SubscribedUnicast,
    SubscribedBroadcast,
    UnsubscribedUnicast,
    UnsubscribedBroadcast,
    SentUnicast,
    PublishedBroadcast,
    Error { error: String },
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum NetworkNotification {
    UnicastMessage {
        topic: String,
        from: pbc::PublicKey,
        data: Vec<u8>,
    },
    BroadcastMessage {
        topic: String,
        data: Vec<u8>,
    },
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RequestKind {
    NetworkRequest(NetworkRequest),
    NodeRequest(NodeRequest),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Request {
    #[serde(flatten)]
    pub kind: RequestKind,
    #[serde(default)]
    #[serde(skip_serializing_if = "is_request_id_default")]
    pub id: u64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ResponseKind {
    NetworkResponse(NetworkResponse),
    NetworkNotification(NetworkNotification),
    NodeResponse(NodeResponse),
    StatusNotification(StatusNotification),
    ChainNotification(ChainNotification),
}

impl yew::agent::Transferable for ResponseKind {}
impl yew::agent::Transferable for RequestKind {}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Response {
    #[serde(flatten)]
    pub kind: ResponseKind,
    #[serde(default)]
    #[serde(skip_serializing_if = "is_request_id_default")]
    pub id: RequestId,
}
