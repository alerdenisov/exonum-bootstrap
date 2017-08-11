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

#[macro_use] extern crate exonum_bootstrap_proc;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate exonum;
#[macro_use] extern crate exonum_bootstrap;

extern crate serde;
extern crate serde_json;
extern crate router;
extern crate bodyparser;
extern crate iron;

use serde_json::Value;
use exonum_bootstrap::transactions::{TransactionMethods};
use exonum_bootstrap::macroses::*;
use exonum_bootstrap::schema::*;
use exonum_bootstrap::explorer::*;
use exonum::blockchain::{self, Schema, Blockchain, Service, GenesisConfig,
                         ValidatorKeys, Transaction, ApiContext};
use exonum::node::{Node, NodeConfig, NodeApiConfig, TransactionSend,
                   ApiSender, NodeChannel};
use exonum::messages::{RawTransaction, FromRaw, Message};
use exonum::storage::{Fork, MemoryDB, MapIndex};
use exonum::crypto::{PublicKey, Hash, HexValue};
use exonum::encoding::{self, Field};
use exonum::api::{Api, ApiError};
use exonum::helpers::fabric::{NodeBuilder, ServiceFactory, Context};
use iron::prelude::*;
use iron::Handler;
use router::Router;

#[derive(exonum_service)]
#[id = "1"]
struct __Currency {
  #[record = "1"]
  wallet: __Wallet,

  // api endpoint for tx (TxCreateWallet)
  #[input = "1"]
  create_wallet: __CreateWallet,

  // api endpoint for tx (TxTransfer)
  #[input = "2"]
  transfer: __Transfer
}

#[derive(exonum_record)]
struct __Wallet {
    #[key]                  // mark field as primary key (will be index of data)
    pub_key: &'static PublicKey,

    #[size = "8"]
    name: &'static str,

    #[config(decimals = "8")]
    #[set]                  // expose setter method `wallet.set_balance(1000u64)`
    balance: u64
}

#[derive(exonum_message)]
struct __CreateWallet {
    #[key]
    pub_key: &'static PublicKey,

    #[size = "8"]
    name: &'static str,
}

#[derive(exonum_message)]
struct __Transfer {
    #[key]
    from: &'static PublicKey,

    to: &'static PublicKey,

    #[config(decimals = "8")]
    amount: u64,

    #[config(random)]
    seed: u64
}

// Implementation of custom Wallet record methods
impl Wallet {
    pub fn increase(&mut self, amount: u64) {
        let balance = self.balance() + amount;

        // Usage of auto-generated setter method
        self.set_balance(balance);
    }

    pub fn decrease(&mut self, amount: u64) {
        let balance = self.balance() - amount;
        self.set_balance(balance);
    }
}

impl TransactionMethods for TxTransfer {
    fn verify(&self) -> bool { (*self.from() != *self.to()) && (self.amount() > 0) }

    fn execute(&self, view: &mut Fork) {
        let mut schema = WalletSchema { view };
        let sender = schema.get(self.from());
        let receiver = schema.get(self.to());
        if let (Some(mut sender), Some(mut receiver)) = (sender, receiver) {
            let amount = self.amount();
            if sender.balance() >= amount {
                sender.decrease(amount);
                receiver.increase(amount);
                let mut wallets = schema.index();
                wallets.put(self.from(), sender);
                wallets.put(self.to(), receiver);
            }
        }
    }

    fn info(&self) -> Value {
      Value::String(format!("Transfer {:?} from {:?} to {:?}", self.amount(), self.from(), self.to()))
    }
}

impl TransactionMethods for TxCreateWallet {
    fn verify(&self) -> bool { true }

    fn execute(&self, view: &mut Fork) {
        let mut schema = WalletSchema { view };
        if schema.get(self.pub_key()).is_none() {
            let wallet = Wallet::new(self.pub_key(), self.name(), 1_5000_000u64);
            println!("Create the wallet: {:?}", wallet);
            schema.index().put(self.pub_key(), wallet)
        }
    }
}

fn main() {
    exonum::helpers::init_logger().unwrap();
    NodeBuilder::new()
        .with_service::<CurrencyService>()
        .run();

        
    println!("Blockchain in ready for transactions!");
}