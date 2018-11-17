//
// MIT License
//
// Copyright (c) 2018 Stegos
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

// #![deny(warnings)]

#[macro_use]
extern crate log;
extern crate bytes;
// #[macro_use]
// extern crate failure_derive;
#[macro_use]
extern crate failure;
extern crate fnv;
extern crate futures;
extern crate ipnetwork;
extern crate libp2p;
extern crate parking_lot;
extern crate pnet;
extern crate protobuf;
extern crate rand;
extern crate stegos_config;
extern crate stegos_crypto;
extern crate stegos_keychain;
extern crate tokio;
extern crate tokio_codec;
extern crate tokio_io;
extern crate unsigned_varint;

mod echo;
mod ncp;
mod node;
mod types;

pub use crate::echo::protocol::{EchoMiddleware, EchoUpgrade};
pub use crate::ncp::protocol;
pub use crate::node::broker::BrokerHandler;
pub use crate::node::Node;
