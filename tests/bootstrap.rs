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
    name: &'static str,

    #[size = "8"]
    #[set]                  // expose setter method `wallet.set_balance(1000u64)`
    balance: u64,

    #[transaction]
    #[id="1"]
    createWallet: TxCreateWallet,

    #[transaction]
    #[id="2"]
    transfer: TxTransfer,
}

// Implementation of custom Wallet record methods
impl<'wallet> Wallet {
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


#[derive(exonum_message)]
#[api="WalletApi"]
#[service="1"]
#[id="1"]
struct __CreateWallet {
    #[key]
    #[size = "32"]
    pub_key: &'static PublicKey,
    
    #[size = "8"]
    name: &'static str,
}

#[derive(exonum_message)]
#[api="WalletApi"]
#[service="1"]
#[id="2"]
struct __Transfer {
    #[key]
    #[size = "32"]
    from: &'static PublicKey,

    #[size = "32"]
    to: &'static PublicKey,

    #[size = "8"]
    amount: u64,

    #[size = "8"]
    seed: u64
}

impl TransactionVerify for TxTransfer {
    fn verify(&self) -> bool {
        (*self.from() != *self.to()) && (self.amount() > 0)
    }
}

impl TransactionExecute for TxTransfer {
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
}

impl TransactionVerify for TxCreateWallet {
    fn verify(&self) -> bool {
        true
    }
}

impl TransactionExecute for TxCreateWallet {
    fn execute(&self, view: &mut Fork) {
        let mut schema = WalletSchema { view };
        if schema.get(self.pub_key()).is_none() {
            let wallet = Wallet::new(self.pub_key(), self.name(), 1000u64);
            println!("Create the wallet: {:?}", wallet);
            schema.index().put(self.pub_key(), wallet)
        }
    }
}

#[test]
fn should_bootstrap() {
    assert_eq!(1, 1);
}