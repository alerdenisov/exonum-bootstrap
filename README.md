# Exonum Bootstrap Library

Main goal to shrink amount of code to bootstap your own blockchain on Exonum Framework.

Make it: 
* Shorter
* Simplier
* Readable

## Features

### Records define
Exonum Bootstrap provides handy proc-macros to define encoded struct (exonum blockchain record):

```rust
// derive macros will create Wallet encoding structure
#[derive(exonum_record)]
struct __Wallet {
  // mark field as primary key (will be index of data)
  #[key]
  pub_key: &'static PublicKey,

  // define custom size
  #[size = "8"]           
  name: &'static str,

  balance: u64
}
```

### Transactions define
Another proc-macros allows to define it same way transactions:

```rust
// macros will create TxCreateWallet structure as encoded message
#[derive(exonum_message)]
struct __CreateWallet {
  #[key]
  pub_key: &'static PublicKey,

  #[size = "8"]
  name: &'static str,
}
```

Also it will create verification of signature inside `verify` function from `Transaction` trait. To create custom verify and\or execute logic please implement `TransactionMethods` trait:

```rust
impl TransactionMethods for TxCreateWallet {
  fn verify(&self) -> bool { 
    self.name().len() > 3
  }

  fn execute(&self, view: &mut Fork) {
    let mut schema = WalletSchema { view };
    if schema.get(self.pub_key()).is_none() {
      let wallet = Wallet::new(self.pub_key(), self.name(), 0);
      schema.index().put(self.pub_key(), wallet)
    }
  }
}
```

### Schema creation
Based on defined records Exonum Bootstrap will create coresponding schemas (like `WalletSchema` for `__Wallet` example).

Base implementation of schema contains two function: `index` and `get`. Function based on `#[key]` signed field of struct and use it as a key value in `MapIndex`

### Service bootstrapping
Exonum Bootstrap could to implement basic service and api for you based on declarative definition:

```rust
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
```

In addition to receiving transactions, the api provides a service scheme:

```json
{
  "schema": "should be here"
}
```

## Roadmap
Future plans to improvement functionality of Exonum Bootstrap pretty opaque and relates to my own need.. but if you wish something special, please, tell me over Issue.

*Future plans:*
- [ ] Outputs (read data from blockchain)
- [ ] Definition of a transactions responses
- [ ] Configuration of API endpoints 