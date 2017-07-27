#[macro_use] extern crate exonum_bootstrap_proc;
#[macro_use] extern crate exonum;
#[macro_use] extern crate exonum_bootstrap;
#[macro_use] extern crate serde_derive;

extern crate serde;
extern crate serde_json;
extern crate router;
extern crate bodyparser;
extern crate iron;

use exonum_bootstrap::transactions::{TransactionVerify,TransactionExecute};
use exonum_bootstrap::macroses::*;
use exonum::blockchain::{self, Blockchain, Service, GenesisConfig,
                         ValidatorKeys, Transaction, ApiContext};
use exonum::node::{Node, NodeConfig, NodeApiConfig, TransactionSend,
                   ApiSender, NodeChannel};
use exonum::messages::{RawTransaction, FromRaw, Message};
use exonum::storage::{Fork, MemoryDB, MapIndex};
use exonum::crypto::{PublicKey, Hash};
use exonum::encoding::{self, Field};
use exonum::api::{Api, ApiError};
use iron::prelude::*;
use iron::Handler;
use router::Router;


#[derive(exonum_record)]
#[service = "1"]            // service ID
#[id = "0"]                 // record ID
struct __Wallet {
    #[key]                  // mark field as primary key (will be index of data)
    #[size = "32"]          // set size in db
    pub_key: &'static PublicKey,

    #[size = "8"]
    #[set]                  // expose setter method `wallet.set_balance(1000u64)`
    balance: u64,

    // #[transaction]
    // #[id="1"]
    // createWallet: TxCreateWallet,
}


// #[derive(exonum_message)]
// #[api="WalletApi"]
// #[service="1"]
// #[id="1"]
// struct __CreateWallet {
//     #[key]
//     #[size = "32"]
//     pub_key: &'static PublicKey,
    
//     #[size = "8"]
//     name: &'static str,
// }

// impl TransactionVerify for TxCreateWallet {
//     fn verify(&self) -> bool {
//         true
//     }
// }

// impl TransactionExecute for TxCreateWallet {
//     fn execute(&self, view: &mut Fork) {
//         let mut schema = WalletSchema { view };
//         if schema.get(self.pub_key()).is_none() {
//             let wallet = Wallet::new(self.pub_key(), self.name(), 1000u64);
//             schema.index().put(self.pub_key(), wallet)
//         }
//     }
// }

#[test]
fn should_bootstrap() {
    assert_eq!(1, 1);
}