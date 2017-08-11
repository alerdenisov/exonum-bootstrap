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
use exonum::storage::{Fork, MemoryDB, MapIndex, StorageValue};
use exonum::crypto::{PublicKey, Hash, HexValue};
use exonum::encoding::{self, Field};
use exonum::api::{Api, ApiError};
use exonum::helpers::fabric::{NodeBuilder, ServiceFactory, Context};
use iron::prelude::*;
use iron::Handler;
use router::Router;

#[derive(exonum_service)]
#[id = "1"]
struct __Productivity {
  #[record]
  account: __Account,

  #[record]
  wallet: __Wallet,

  #[record]
  task: __Task,

  #[record]
  activity: __Activity,

  // api endpoint for tx (TxCreateWallet)
  #[input]
  create_wallet: __CreateAccount,

  #[input]
  create_task: __CreateTask,

  #[input]
  create_activity: __CreateActivity
}

#[derive(exonum_record)]
struct __Account {
  #[key]
  pub_key: &'static PublicKey,

  #[set]
  #[size = "8"]
  name: &'static str,
}

#[derive(exonum_record)]
struct __Wallet {
  #[key]
  pub_key: &'static PublicKey,

  #[set]
  balance: u64,
}

#[derive(exonum_record)]
struct __Activity {
  #[key]
  key: &'static Hash,

  task_hash: &'static Hash,

  worker: &'static PublicKey,

  #[size = "256"]
  data: &'static str
}

#[derive(exonum_record)]
struct __Task {
  #[key]
  key: &'static Hash,

  author: &'static PublicKey,  

  worker: &'static PublicKey,

  #[set]
  #[size = "128"]
  name: &'static str,

  #[set]
  open: bool,
}


#[derive(exonum_message)]
struct __CreateAccount {
  #[key]
  #[config(auth)]
  pub_key: &'static PublicKey,

  #[size = "8"]
  name: &'static str,
}

#[derive(exonum_message)]
struct __CreateTask {
  #[key]
  #[config(auth)]
  pub_key: &'static PublicKey,

  #[config(minLength = "3")]
  #[size = "24"]
  name: &'static str,

  #[config(random)]
  seed: u64
}

#[derive(exonum_message)]
struct __CreateActivity {
  #[key]
  #[config(auth)]
  pub_key: &'static PublicKey,

  task_hash: &'static Hash,

  #[size = "256"]
  map: &'static str,

  #[config(random)]
  seed: u64
}

impl TransactionMethods for TxCreateAccount {
    fn execute(&self, view: &mut Fork) {
      {
        let mut accounts_schema = AccountSchema { view };
        if accounts_schema.get(self.pub_key()).is_none() {
          let account = Account::new(self.pub_key(), self.name()); 
          accounts_schema.index().put(self.pub_key(), account);
        } else {
          panic!("Account is already created");
        }
      }
      {
        let mut wallets_schema = WalletSchema { view };
        let wallet = Wallet::new(self.pub_key(), 0u64);         
        wallets_schema.index().put(self.pub_key(), wallet);
      }
    }
}

impl TransactionMethods for TxCreateTask {
  fn verify(&self) -> bool {
    self.name().len() > 3
  }

  fn execute(&self, view: &mut Fork) {
    let mut schema = TaskSchema { view };
    let task_hash = &StorageValue::hash(&format!("{}_{}", self.pub_key().to_hex(), self.name())); 
    
    if schema.get(task_hash).is_none() {
      let task = Task::new(task_hash, self.pub_key(), &PublicKey::zero(), self.name(), false);
      schema.index().put(task_hash, task);
    }        
  }
}

impl TransactionMethods for TxCreateActivity {
  fn execute(&self, view: &mut Fork) {
    {
      let mut task_schema = TaskSchema { view };
      
      let task = task_schema.get(self.task_hash());
      if task.is_none() {
        panic!("Target task not found");
      }
    }

    let mut schema = ActivitySchema { view };
    let activity_hash = &StorageValue::hash(&format!("activity_{}_{}_{}", self.pub_key().to_hex(), self.task_hash().to_hex(), self.seed()));
    let activity = Activity::new(activity_hash, self.task_hash(), self.pub_key(), self.map());
    schema.index().put(activity_hash, activity);
  }
}

fn main() {
    exonum::helpers::init_logger().unwrap();
    NodeBuilder::new()
        .with_service::<ProductivityService>()
        .run();

        
    println!("Blockchain in ready for transactions!");
}