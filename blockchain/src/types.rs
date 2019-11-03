//
// Copyright (c) 2019 Stegos AG
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
use crate::awards::Awards;
use crate::block::MicroBlock;
use serde_derive::{Deserialize, Serialize};
use stegos_crypto::{
    hash::{Hash, Hashable, Hasher},
    pbc, scc,
};

pub type ViewCounter = u32;
pub type ValidatorId = u32;

/// Saved information about validator, and its slotcount in epoch.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ValidatorKeyInfo {
    pub(crate) network_pkey: pbc::PublicKey,
    pub(crate) account_pkey: scc::PublicKey,
    pub(crate) slots: i64,
}

/// Information about service award payout.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PayoutInfo {
    pub(crate) recipient: scc::PublicKey,
    pub(crate) amount: i64,
}
/// Full information about service award state.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct AwardsInfo {
    #[serde(flatten)]
    pub service_award_state: Awards,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payout: Option<PayoutInfo>,
}

/// Retrospective information for some epoch.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct EpochInfo {
    pub validators: Vec<ValidatorKeyInfo>,
    pub facilitator: pbc::PublicKey,
    pub awards: AwardsInfo,
}

/// Information of current chain, that is used as proof of viewchange.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ChainInfo {
    pub epoch: u64,
    pub offset: u32,
    pub view_change: ViewCounter,
    pub last_block: Hash,
}

impl ChainInfo {
    /// Create ChainInfo from micro block.
    /// ## Panics
    /// if view_change is equal to 0
    pub fn from_micro_block(micro_block: &MicroBlock) -> Self {
        assert_ne!(micro_block.header.view_change, 0);
        ChainInfo {
            epoch: micro_block.header.epoch,
            offset: micro_block.header.offset,
            view_change: micro_block.header.view_change - 1,
            last_block: micro_block.header.previous,
        }
    }

    /// Create ChainInfo from blockchain.
    #[cfg(feature = "logic")]
    pub fn from_blockchain(blockchain: &crate::Blockchain) -> Self {
        ChainInfo {
            epoch: blockchain.epoch(),
            offset: blockchain.offset(),
            view_change: blockchain.view_change(),
            last_block: blockchain.last_block_hash(),
        }
    }
}

impl Hashable for ChainInfo {
    fn hash(&self, hasher: &mut Hasher) {
        self.epoch.hash(hasher);
        self.offset.hash(hasher);
        self.view_change.hash(hasher);
        self.last_block.hash(hasher);
    }
}

/// A helper to find UTXO in this blockchain.
#[derive(Eq, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub(crate) enum OutputKey {
    MacroBlock {
        /// Block Epoch.
        epoch: u64,
        /// Output number.
        output_id: u32,
    },
    MicroBlock {
        /// Block Epoch.
        epoch: u64,
        /// Block Height.
        offset: u32,
        /// Transaction number.
        tx_id: u32,
        /// Output number.
        txout_id: u32,
    },
}

#[derive(Debug, Default, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct LSN(pub(crate) u64, pub(crate) u32); // use `struct` to disable explicit casts.
