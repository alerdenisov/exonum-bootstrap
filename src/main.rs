#[macro_use] extern crate exonum_bootstrap_proc;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate exonum;
#[macro_use] extern crate exonum_bootstrap;

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
    #[ty="TxCreateWallet"]
    createWallet: bool,

    #[transaction]
    #[id="2"]
    #[ty="TxTransfer"]
    transfer: bool,
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

// // // // // // // // // // REST API // // // // // // // // // //
// #[derive(Clone)]
// struct CryptocurrencyApi {
//     channel: ApiSender<NodeChannel>,
// }

// impl Api for CryptocurrencyApi {
//     fn wire(&self, router: &mut Router) {

//         #[serde(untagged)]
//         #[derive(Clone, Serialize, Deserialize)]
//         enum TransactionRequest {
//             CreateWallet(TxCreateWallet),
//             Transfer(TxTransfer),
//         }

//         impl Into<Box<Transaction>> for TransactionRequest {
//             fn into(self) -> Box<Transaction> {
//                 match self {
//                     TransactionRequest::CreateWallet(trans) => Box::new(trans),
//                     TransactionRequest::Transfer(trans) => Box::new(trans),
//                 }
//             }
//         }

//         #[derive(Serialize, Deserialize)]
//         struct TransactionResponse {
//             tx_hash: Hash,
//         }

//         let self_ = self.clone();
//         let transaction = move |req: &mut Request| -> IronResult<Response> {
//             match req.get::<bodyparser::Struct<TransactionRequest>>() {
//                 Ok(Some(transaction)) => {
//                     let transaction: Box<Transaction> = transaction.into();
//                     let tx_hash = transaction.hash();
//                     self_.channel.send(transaction).map_err(|e| ApiError::Events(e))?;
//                     let json = TransactionResponse { tx_hash };
//                     self_.ok_response(&serde_json::to_value(&json).unwrap())
//                 }
//                 Ok(None) => Err(ApiError::IncorrectRequest("Empty request body".into()))?,
//                 Err(e) => Err(ApiError::IncorrectRequest(Box::new(e)))?,
//             }
//         };
//         let route_post = "/v1/wallets/transaction";
//         router.post(&route_post, transaction, "transaction");
//     }
// }


// // // // // // // // // // SERVICE DECLARATION // // // // // // // // // //
// struct CurrencyService;

// impl Service for CurrencyService {
//     fn service_name(&self) -> &'static str { "cryptocurrency" }

//     fn service_id(&self) -> u16 { 1 }

//     fn tx_from_raw(&self, raw: RawTransaction) -> Result<Box<Transaction>, encoding::Error> {
//         let trans: Box<Transaction> = match raw.message_type() {
//             1 => Box::new(TxTransfer::from_raw(raw)?),
//             2 => Box::new(TxCreateWallet::from_raw(raw)?),
//             _ => {
//                 return Err(encoding::Error::IncorrectMessageType { message_type: raw.message_type() });
//             },
//         };
//         Ok(trans)
//     }

//     fn public_api_handler(&self, ctx: &ApiContext) -> Option<Box<Handler>> {
//         let mut router = Router::new();
//         let api = CryptocurrencyApi {
//             channel: ctx.node_channel().clone(),
//         };
//         api.wire(&mut router);
//         Some(Box::new(router))
//     }
// }

// // // // // // // // // // // ENTRY POINT // // // // // // // // // //

fn main() {
    exonum::helpers::init_logger().unwrap();

    println!("Creating in-memory database...");
    let db = MemoryDB::new();
    let services: Vec<Box<Service>> = vec![
        Box::new(WalletService),
    ];
    let blockchain = Blockchain::new(Box::new(db), services);

    let (consensus_public_key, consensus_secret_key) = exonum::crypto::gen_keypair();
    let (service_public_key, service_secret_key) = exonum::crypto::gen_keypair();

    let peer_address = "0.0.0.0:2000".parse().unwrap();
    let api_address = "0.0.0.0:8000".parse().unwrap();

    let validator_keys = ValidatorKeys {
        consensus_key: consensus_public_key,
        service_key: service_public_key,
    };
    let genesis = GenesisConfig::new(vec![validator_keys].into_iter());

    let api_cfg = NodeApiConfig {
        public_api_address: Some(api_address),
        ..Default::default()
    };

    let node_cfg = NodeConfig {
        listen_address: peer_address,
        peers: vec![],
        service_public_key,
        service_secret_key,
        consensus_public_key,
        consensus_secret_key,
        genesis,
        external_address: None,
        network: Default::default(),
        whitelist: Default::default(),
        api: api_cfg,
        mempool: Default::default(),
        services_configs: Default::default(),
    };

    println!("Starting a single node...");
    let mut node = Node::new(blockchain, node_cfg);

    println!("Blockchain in ready for transactions!");
    node.run().unwrap();
}

// fn main() {
    
// }