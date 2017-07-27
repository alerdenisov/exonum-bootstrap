extern crate iron;
extern crate mount;
extern crate logger;
extern crate serde;
extern crate juniper;

use std::env;

use mount::Mount;
use logger::Logger;
use iron::prelude::*;
use juniper::EmptyMutation;
use juniper::iron_handlers::{GraphQLHandler, GraphiQLHandler};
use juniper::tests::model::Database;

macro_rules! service_data {
    (
        $(#[$attr:meta])*
        struct $name:ident {
            
        }
    ) => {
        
    };
}

service_data! {
    struct Wallet {
        key pub_key:      &PublicKey  [32]        
        get name:         &str        [8]        
        set balance:      u64         [8]
    }

    transaction Create {
        field pub_key:   &PublicKey   [32]
        field name:      &str         [8]

        fn execute(&self, view: &mut Fork) {
            let mut schema = WalletSchema { view };
            if schema.get(self.pub_key()).is_none() {
                let wallet = Wallet::new(self.pub_key(), self.name(), 100);
                println!("Create the wallet: {:?}", wallet);
                schema.index().put(self.pub_key(), wallet)
            }
        }
    }

    transaction Transfer {
        field from:      &PublicKey   [32]
        field to:        &PublicKey   [32]
        field amount:    u64          [8]
        field seed:      u64          [8]

        fn verify(&self) -> bool {
            self.amount > 0
        }

        fn execute(&self, view: &mut Fork) {
            let mut schema = WalletSchema { view };
            let sender = schema.get(self.from());
            let receiver = schema.get(self.to());
            if let (Some(mut sender), Some(mut receiver)) = (sender, receiver) {
                let amount = self.amount();
                if sender.balance() >= amount {
                    sender.balance(sender.balance() - amount)
                    receiver.balance(receiver.balance() + amount);
                    println!("Transfer between wallets: {:?} => {:?}", sender, receiver);
                    let mut wallets = schema.index();
                    wallets.put(self.from(), sender);
                    wallets.put(self.to(), receiver);
                }
            }
        }
    }
}


fn context_factory(_: &mut Request) -> Database {
    Database::new()
}

fn main() {
    let mut mount = Mount::new();

    let graphql_endpoint = GraphQLHandler::new(
        context_factory,
        Database::new(),
        EmptyMutation::<Database>::new(),
    );
    let graphiql_endpoint = GraphiQLHandler::new("/graphql");

    mount.mount("/", graphiql_endpoint);
    mount.mount("/graphql", graphql_endpoint);

    let (logger_before, logger_after) = Logger::new(None);

    let mut chain = Chain::new(mount);
    chain.link_before(logger_before);
    chain.link_after(logger_after);

    let host = env::var("LISTEN").unwrap_or("0.0.0.0:8080".to_owned());
    println!("GraphQL server started on {}", host);
    Iron::new(chain).http(host.as_str()).unwrap();
}