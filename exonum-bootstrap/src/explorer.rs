// Copyright (c) 2017 Aler Denisov <aler.zampillo@gmail.com>

// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:

// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.

// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use serde_json::Value;
use std::cmp;
use exonum::storage::ListProof;
use exonum::crypto::Hash;
use exonum::blockchain::{Schema, Blockchain, Block, TxLocation};
use exonum::messages::Precommit;

// TODO: if explorer is usable anywhere else, remove `ApiError` dependencies.
use exonum::api::ApiError;

#[derive(Debug)]
pub struct Explorer<'explorer> {
  blockchain: &'explorer Blockchain,
}

/// Block information.
#[derive(Debug, Serialize)]
pub struct BlockInfo {
    block: Block,
    precommits: Vec<Precommit>,
    txs: Vec<Hash>,
}

/// Transaction information.
#[derive(Debug, Serialize)]
pub struct TxInfo {
    content: Value,
    tx_index: u64,
    block_height: u64,
    proof_to_block_merkle_root: ListProof<Hash>,
}

impl<'explorer> Explorer<'explorer> {
  pub fn new(blockchain: &'explorer Blockchain) -> Self {
    Explorer { blockchain: blockchain }
  }

  pub fn tx_info(&self, tx_hash: &'explorer Hash) -> Result<Option<TxInfo>, ApiError> {
    let b = self.blockchain.clone();
    let snapshot = b.snapshot();
    let schema = Schema::new(&snapshot);
    let tx = schema.transactions().get(tx_hash);
    let res = match tx {
        None => None,
        Some(raw_tx) => {
          let box_transaction = self.blockchain.tx_from_raw(raw_tx.clone()).ok_or_else(|| {
            ApiError::Service(format!("Service not found for tx: {:?}", raw_tx).into())
          })?;
          
          let content = box_transaction.info();

          let location = schema.tx_location_by_tx_hash().get(tx_hash).expect(
            &format!(
              "Not found tx_hash location: {:?}",
              tx_hash
            ),
          );

          let block_height = location.block_height();
          let tx_index = location.position_in_block();
          let proof = schema.block_txs(block_height).get_proof(tx_index);
          let tx_info = TxInfo {
            block_height: block_height,
            tx_index: tx_index,
            content: content,
            proof_to_block_merkle_root: proof,
          };
          Some(tx_info)
        }
    };
    Ok(res)
  }
}