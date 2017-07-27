#![feature(prelude_import)]
#![no_std]
#[prelude_import]
use std::prelude::v1::*;
#[macro_use]
extern crate std as std;
#[macro_use]
extern crate exonum_bootstrap_proc;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate exonum;
#[macro_use]
extern crate exonum_bootstrap;

extern crate serde;
extern crate serde_json;
extern crate router;
extern crate bodyparser;
extern crate iron;

use exonum_bootstrap::transactions::{TransactionVerify, TransactionExecute};
use exonum_bootstrap::macroses::*;
use exonum_bootstrap::schema::*;
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

// service ID
// record ID
#[service = "1"]
#[id = "0"]
struct __Wallet {
    // mark field as primary key (will be index of data)
    // set size in db
    #[key]
    #[size = "32"]
    pub_key: &'static PublicKey,

    #[size = "8"]
    name: &'static str,

    // expose setter method `wallet.set_balance(1000u64)`
    #[size = "8"]
    #[config(max = "10000", min = "0")]
    #[set]
    balance: u64,

    #[transaction]
    #[id = "1"]
    create_wallet: TxCreateWallet,

    #[transaction]
    #[id = "2"]
    transfer: TxTransfer,
}

// Implementation of custom Wallet record methods

// Usage of auto-generated setter method













// // // // // // // // // // // ENTRY POINT // // // // // // // // // //










// fn main() {

// }
pub struct Wallet {
    raw: Vec<u8>,
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::clone::Clone for Wallet {
    #[inline]
    fn clone(&self) -> Wallet {
        match *self {
            Wallet { raw: ref __self_0_0 } =>
            Wallet{raw: ::std::clone::Clone::clone(&(*__self_0_0)),},
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::cmp::PartialEq for Wallet {
    #[inline]
    fn eq(&self, __arg_0: &Wallet) -> bool {
        match *__arg_0 {
            Wallet { raw: ref __self_1_0 } =>
            match *self {
                Wallet { raw: ref __self_0_0 } =>
                true && (*__self_0_0) == (*__self_1_0),
            },
        }
    }
    #[inline]
    fn ne(&self, __arg_0: &Wallet) -> bool {
        match *__arg_0 {
            Wallet { raw: ref __self_1_0 } =>
            match *self {
                Wallet { raw: ref __self_0_0 } =>
                false || (*__self_0_0) != (*__self_1_0),
            },
        }
    }
}
impl <'a> ::encoding::Field<'a> for Wallet {
    unsafe fn read(buffer: &'a [u8], from: ::encoding::Offset,
                   to: ::encoding::Offset) -> Self {
        let vec: Vec<u8> = ::encoding::Field::read(buffer, from, to);
        ::storage::StorageValue::from_bytes(::std::borrow::Cow::Owned(vec))
    }
    fn write(&self, buffer: &mut Vec<u8>, from: ::encoding::Offset,
             to: ::encoding::Offset) {
        ::encoding::Field::write(&self.raw, buffer, from, to);
    }
    #[allow(unused_variables)]
    fn check(buffer: &'a [u8], from_st_val: ::encoding::CheckedOffset,
             to_st_val: ::encoding::CheckedOffset,
             latest_segment: ::encoding::CheckedOffset)
     -> ::encoding::Result {
        let latest_segment_origin =
            <&[u8] as
                ::encoding::Field>::check(buffer, from_st_val, to_st_val,
                                          latest_segment)?;
        let vec: &[u8] =
            unsafe {
                ::encoding::Field::read(buffer,
                                        from_st_val.unchecked_offset(),
                                        to_st_val.unchecked_offset())
            };
        let latest_segment: ::encoding::CheckedOffset =
            (48 as ::encoding::Offset).into();
        let latest_segment =
            <&PublicKey as
                ::encoding::Field>::check(&vec, 0.into(), 32.into(),
                                          latest_segment)?;
        let latest_segment =
            <&str as
                ::encoding::Field>::check(&vec, 32.into(), 40.into(),
                                          latest_segment)?;
        let latest_segment =
            <u64 as
                ::encoding::Field>::check(&vec, 40.into(), 48.into(),
                                          latest_segment)?;
        Ok(latest_segment_origin)
    }
    fn field_size() -> ::encoding::Offset { 8 as ::encoding::Offset }
}
impl ::storage::StorageValue for Wallet {
    fn into_bytes(self) -> Vec<u8> { self.raw }
    fn from_bytes(v: ::std::borrow::Cow<[u8]>) -> Self {
        Wallet{raw: v.into_owned(),}
    }
    fn hash(&self) -> ::crypto::Hash { Wallet::hash(self) }
}
impl Wallet {
    #[allow(unused_imports, unused_mut)]
    /// Creates a new instance with given parameters.
    pub fn new(pub_key: &PublicKey, name: &str, balance: u64) -> Wallet {
        {
            use ::encoding::Field;
            if true {
                {
                    match (&(0), &(0)) {
                        (left_val, right_val) => {
                            if !(*left_val == *right_val) {
                                {
                                    ::rt::begin_panic_fmt(&::std::fmt::Arguments::new_v1({
                                                                                             static __STATIC_FMTSTR:
                                                                                                    &'static [&'static str]
                                                                                                    =
                                                                                                 &["assertion failed: `(left == right)`\n  left: `",
                                                                                                   "`,\n right: `",
                                                                                                   "`: "];
                                                                                             __STATIC_FMTSTR
                                                                                         },
                                                                                         &match (&left_val,
                                                                                                 &right_val,
                                                                                                 &::std::fmt::Arguments::new_v1({
                                                                                                                                    static __STATIC_FMTSTR:
                                                                                                                                           &'static [&'static str]
                                                                                                                                           =
                                                                                                                                        &["first field should start from 0"];
                                                                                                                                    __STATIC_FMTSTR
                                                                                                                                },
                                                                                                                                &match ()
                                                                                                                                     {
                                                                                                                                     ()
                                                                                                                                     =>
                                                                                                                                     [],
                                                                                                                                 }))
                                                                                              {
                                                                                              (__arg0,
                                                                                               __arg1,
                                                                                               __arg2)
                                                                                              =>
                                                                                              [::std::fmt::ArgumentV1::new(__arg0,
                                                                                                                           ::std::fmt::Debug::fmt),
                                                                                               ::std::fmt::ArgumentV1::new(__arg1,
                                                                                                                           ::std::fmt::Debug::fmt),
                                                                                               ::std::fmt::ArgumentV1::new(__arg2,
                                                                                                                           ::std::fmt::Display::fmt)],
                                                                                          }),
                                                          {
                                                              static _FILE_LINE_COL:
                                                                     (&'static str,
                                                                      u32,
                                                                      u32) =
                                                                  ("src/main.rs",
                                                                   28u32,
                                                                   9u32);
                                                              &_FILE_LINE_COL
                                                          })
                                }
                            }
                        }
                    }
                };
            };
            if true {
                {
                    match (&(32 - 0), &(<&PublicKey as Field>::field_size()))
                        {
                        (left_val, right_val) => {
                            if !(*left_val == *right_val) {
                                {
                                    ::rt::begin_panic_fmt(&::std::fmt::Arguments::new_v1({
                                                                                             static __STATIC_FMTSTR:
                                                                                                    &'static [&'static str]
                                                                                                    =
                                                                                                 &["assertion failed: `(left == right)`\n  left: `",
                                                                                                   "`,\n right: `",
                                                                                                   "`: "];
                                                                                             __STATIC_FMTSTR
                                                                                         },
                                                                                         &match (&left_val,
                                                                                                 &right_val,
                                                                                                 &::std::fmt::Arguments::new_v1({
                                                                                                                                    static __STATIC_FMTSTR:
                                                                                                                                           &'static [&'static str]
                                                                                                                                           =
                                                                                                                                        &["wrong size of field"];
                                                                                                                                    __STATIC_FMTSTR
                                                                                                                                },
                                                                                                                                &match ()
                                                                                                                                     {
                                                                                                                                     ()
                                                                                                                                     =>
                                                                                                                                     [],
                                                                                                                                 }))
                                                                                              {
                                                                                              (__arg0,
                                                                                               __arg1,
                                                                                               __arg2)
                                                                                              =>
                                                                                              [::std::fmt::ArgumentV1::new(__arg0,
                                                                                                                           ::std::fmt::Debug::fmt),
                                                                                               ::std::fmt::ArgumentV1::new(__arg1,
                                                                                                                           ::std::fmt::Debug::fmt),
                                                                                               ::std::fmt::ArgumentV1::new(__arg2,
                                                                                                                           ::std::fmt::Display::fmt)],
                                                                                          }),
                                                          {
                                                              static _FILE_LINE_COL:
                                                                     (&'static str,
                                                                      u32,
                                                                      u32) =
                                                                  ("src/main.rs",
                                                                   28u32,
                                                                   9u32);
                                                              &_FILE_LINE_COL
                                                          })
                                }
                            }
                        }
                    }
                };
            };
            if true {
                {
                    match (&(32), &(32)) {
                        (left_val, right_val) => {
                            if !(*left_val == *right_val) {
                                {
                                    ::rt::begin_panic_fmt(&::std::fmt::Arguments::new_v1({
                                                                                             static __STATIC_FMTSTR:
                                                                                                    &'static [&'static str]
                                                                                                    =
                                                                                                 &["assertion failed: `(left == right)`\n  left: `",
                                                                                                   "`,\n right: `",
                                                                                                   "`: "];
                                                                                             __STATIC_FMTSTR
                                                                                         },
                                                                                         &match (&left_val,
                                                                                                 &right_val,
                                                                                                 &::std::fmt::Arguments::new_v1({
                                                                                                                                    static __STATIC_FMTSTR:
                                                                                                                                           &'static [&'static str]
                                                                                                                                           =
                                                                                                                                        &["fields should be adjacent"];
                                                                                                                                    __STATIC_FMTSTR
                                                                                                                                },
                                                                                                                                &match ()
                                                                                                                                     {
                                                                                                                                     ()
                                                                                                                                     =>
                                                                                                                                     [],
                                                                                                                                 }))
                                                                                              {
                                                                                              (__arg0,
                                                                                               __arg1,
                                                                                               __arg2)
                                                                                              =>
                                                                                              [::std::fmt::ArgumentV1::new(__arg0,
                                                                                                                           ::std::fmt::Debug::fmt),
                                                                                               ::std::fmt::ArgumentV1::new(__arg1,
                                                                                                                           ::std::fmt::Debug::fmt),
                                                                                               ::std::fmt::ArgumentV1::new(__arg2,
                                                                                                                           ::std::fmt::Display::fmt)],
                                                                                          }),
                                                          {
                                                              static _FILE_LINE_COL:
                                                                     (&'static str,
                                                                      u32,
                                                                      u32) =
                                                                  ("src/main.rs",
                                                                   28u32,
                                                                   9u32);
                                                              &_FILE_LINE_COL
                                                          })
                                }
                            }
                        }
                    }
                };
            };
            if true {
                {
                    match (&(40 - 32), &(<&str as Field>::field_size())) {
                        (left_val, right_val) => {
                            if !(*left_val == *right_val) {
                                {
                                    ::rt::begin_panic_fmt(&::std::fmt::Arguments::new_v1({
                                                                                             static __STATIC_FMTSTR:
                                                                                                    &'static [&'static str]
                                                                                                    =
                                                                                                 &["assertion failed: `(left == right)`\n  left: `",
                                                                                                   "`,\n right: `",
                                                                                                   "`: "];
                                                                                             __STATIC_FMTSTR
                                                                                         },
                                                                                         &match (&left_val,
                                                                                                 &right_val,
                                                                                                 &::std::fmt::Arguments::new_v1({
                                                                                                                                    static __STATIC_FMTSTR:
                                                                                                                                           &'static [&'static str]
                                                                                                                                           =
                                                                                                                                        &["wrong size of field"];
                                                                                                                                    __STATIC_FMTSTR
                                                                                                                                },
                                                                                                                                &match ()
                                                                                                                                     {
                                                                                                                                     ()
                                                                                                                                     =>
                                                                                                                                     [],
                                                                                                                                 }))
                                                                                              {
                                                                                              (__arg0,
                                                                                               __arg1,
                                                                                               __arg2)
                                                                                              =>
                                                                                              [::std::fmt::ArgumentV1::new(__arg0,
                                                                                                                           ::std::fmt::Debug::fmt),
                                                                                               ::std::fmt::ArgumentV1::new(__arg1,
                                                                                                                           ::std::fmt::Debug::fmt),
                                                                                               ::std::fmt::ArgumentV1::new(__arg2,
                                                                                                                           ::std::fmt::Display::fmt)],
                                                                                          }),
                                                          {
                                                              static _FILE_LINE_COL:
                                                                     (&'static str,
                                                                      u32,
                                                                      u32) =
                                                                  ("src/main.rs",
                                                                   28u32,
                                                                   9u32);
                                                              &_FILE_LINE_COL
                                                          })
                                }
                            }
                        }
                    }
                };
            };
            if true {
                {
                    match (&(40), &(40)) {
                        (left_val, right_val) => {
                            if !(*left_val == *right_val) {
                                {
                                    ::rt::begin_panic_fmt(&::std::fmt::Arguments::new_v1({
                                                                                             static __STATIC_FMTSTR:
                                                                                                    &'static [&'static str]
                                                                                                    =
                                                                                                 &["assertion failed: `(left == right)`\n  left: `",
                                                                                                   "`,\n right: `",
                                                                                                   "`: "];
                                                                                             __STATIC_FMTSTR
                                                                                         },
                                                                                         &match (&left_val,
                                                                                                 &right_val,
                                                                                                 &::std::fmt::Arguments::new_v1({
                                                                                                                                    static __STATIC_FMTSTR:
                                                                                                                                           &'static [&'static str]
                                                                                                                                           =
                                                                                                                                        &["fields should be adjacent"];
                                                                                                                                    __STATIC_FMTSTR
                                                                                                                                },
                                                                                                                                &match ()
                                                                                                                                     {
                                                                                                                                     ()
                                                                                                                                     =>
                                                                                                                                     [],
                                                                                                                                 }))
                                                                                              {
                                                                                              (__arg0,
                                                                                               __arg1,
                                                                                               __arg2)
                                                                                              =>
                                                                                              [::std::fmt::ArgumentV1::new(__arg0,
                                                                                                                           ::std::fmt::Debug::fmt),
                                                                                               ::std::fmt::ArgumentV1::new(__arg1,
                                                                                                                           ::std::fmt::Debug::fmt),
                                                                                               ::std::fmt::ArgumentV1::new(__arg2,
                                                                                                                           ::std::fmt::Display::fmt)],
                                                                                          }),
                                                          {
                                                              static _FILE_LINE_COL:
                                                                     (&'static str,
                                                                      u32,
                                                                      u32) =
                                                                  ("src/main.rs",
                                                                   28u32,
                                                                   9u32);
                                                              &_FILE_LINE_COL
                                                          })
                                }
                            }
                        }
                    }
                };
            };
            if true {
                {
                    match (&(48), &(48)) {
                        (left_val, right_val) => {
                            if !(*left_val == *right_val) {
                                {
                                    ::rt::begin_panic_fmt(&::std::fmt::Arguments::new_v1({
                                                                                             static __STATIC_FMTSTR:
                                                                                                    &'static [&'static str]
                                                                                                    =
                                                                                                 &["assertion failed: `(left == right)`\n  left: `",
                                                                                                   "`,\n right: `",
                                                                                                   "`: "];
                                                                                             __STATIC_FMTSTR
                                                                                         },
                                                                                         &match (&left_val,
                                                                                                 &right_val,
                                                                                                 &::std::fmt::Arguments::new_v1({
                                                                                                                                    static __STATIC_FMTSTR:
                                                                                                                                           &'static [&'static str]
                                                                                                                                           =
                                                                                                                                        &["last field should matches the size of struct"];
                                                                                                                                    __STATIC_FMTSTR
                                                                                                                                },
                                                                                                                                &match ()
                                                                                                                                     {
                                                                                                                                     ()
                                                                                                                                     =>
                                                                                                                                     [],
                                                                                                                                 }))
                                                                                              {
                                                                                              (__arg0,
                                                                                               __arg1,
                                                                                               __arg2)
                                                                                              =>
                                                                                              [::std::fmt::ArgumentV1::new(__arg0,
                                                                                                                           ::std::fmt::Debug::fmt),
                                                                                               ::std::fmt::ArgumentV1::new(__arg1,
                                                                                                                           ::std::fmt::Debug::fmt),
                                                                                               ::std::fmt::ArgumentV1::new(__arg2,
                                                                                                                           ::std::fmt::Display::fmt)],
                                                                                          }),
                                                          {
                                                              static _FILE_LINE_COL:
                                                                     (&'static str,
                                                                      u32,
                                                                      u32) =
                                                                  ("src/main.rs",
                                                                   28u32,
                                                                   9u32);
                                                              &_FILE_LINE_COL
                                                          })
                                }
                            }
                        }
                    }
                };
            };
            if true {
                {
                    match (&(48 - 40), &(<u64 as Field>::field_size())) {
                        (left_val, right_val) => {
                            if !(*left_val == *right_val) {
                                {
                                    ::rt::begin_panic_fmt(&::std::fmt::Arguments::new_v1({
                                                                                             static __STATIC_FMTSTR:
                                                                                                    &'static [&'static str]
                                                                                                    =
                                                                                                 &["assertion failed: `(left == right)`\n  left: `",
                                                                                                   "`,\n right: `",
                                                                                                   "`: "];
                                                                                             __STATIC_FMTSTR
                                                                                         },
                                                                                         &match (&left_val,
                                                                                                 &right_val,
                                                                                                 &::std::fmt::Arguments::new_v1({
                                                                                                                                    static __STATIC_FMTSTR:
                                                                                                                                           &'static [&'static str]
                                                                                                                                           =
                                                                                                                                        &["wrong size of field"];
                                                                                                                                    __STATIC_FMTSTR
                                                                                                                                },
                                                                                                                                &match ()
                                                                                                                                     {
                                                                                                                                     ()
                                                                                                                                     =>
                                                                                                                                     [],
                                                                                                                                 }))
                                                                                              {
                                                                                              (__arg0,
                                                                                               __arg1,
                                                                                               __arg2)
                                                                                              =>
                                                                                              [::std::fmt::ArgumentV1::new(__arg0,
                                                                                                                           ::std::fmt::Debug::fmt),
                                                                                               ::std::fmt::ArgumentV1::new(__arg1,
                                                                                                                           ::std::fmt::Debug::fmt),
                                                                                               ::std::fmt::ArgumentV1::new(__arg2,
                                                                                                                           ::std::fmt::Display::fmt)],
                                                                                          }),
                                                          {
                                                              static _FILE_LINE_COL:
                                                                     (&'static str,
                                                                      u32,
                                                                      u32) =
                                                                  ("src/main.rs",
                                                                   28u32,
                                                                   9u32);
                                                              &_FILE_LINE_COL
                                                          })
                                }
                            }
                        }
                    }
                };
            };
        };
        let mut buf = ::vec::from_elem(0, 48);
        ::encoding::Field::write(&pub_key, &mut buf, 0, 32);
        ::encoding::Field::write(&name, &mut buf, 32, 40);
        ::encoding::Field::write(&balance, &mut buf, 40, 48);
        Wallet{raw: buf,}
    }
    /// Hashes data as a raw byte array and returns the resulting hash.
    pub fn hash(&self) -> ::crypto::Hash { ::crypto::hash(self.raw.as_ref()) }
    pub fn pub_key(&self) -> &PublicKey {
        use ::encoding::Field;
        unsafe { Field::read(&self.raw, 0, 32) }
    }
    pub fn name(&self) -> &str {
        use ::encoding::Field;
        unsafe { Field::read(&self.raw, 32, 40) }
    }
    pub fn balance(&self) -> u64 {
        use ::encoding::Field;
        unsafe { Field::read(&self.raw, 40, 48) }
    }
}
impl ::std::fmt::Debug for Wallet {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter)
     -> Result<(), ::std::fmt::Error> {
        fmt.debug_struct("Wallet").field("pub_key",
                                         &self.pub_key()).field("name",
                                                                &self.name()).field("balance",
                                                                                    &self.balance()).finish()
    }
}
impl ::encoding::serialize::json::ExonumJson for Wallet {
    #[allow(unused_variables)]
    fn deserialize_field<B>(value:
                                &::encoding::serialize::json::reexport::Value,
                            buffer: &mut B, from: ::encoding::Offset,
                            _to: ::encoding::Offset)
     -> Result<(), Box<::std::error::Error>> where
     B: ::encoding::serialize::WriteBufferWrapper {
        let obj = value.as_object().ok_or("Can\'t cast json as object.")?;
        let val = obj.get("pub_key").ok_or("Can\'t get object from json.")?;
        <&PublicKey as
            ::encoding::serialize::json::ExonumJson>::deserialize_field(val,
                                                                        buffer,
                                                                        from +
                                                                            0,
                                                                        from +
                                                                            32)?;
        let val = obj.get("name").ok_or("Can\'t get object from json.")?;
        <&str as
            ::encoding::serialize::json::ExonumJson>::deserialize_field(val,
                                                                        buffer,
                                                                        from +
                                                                            32,
                                                                        from +
                                                                            40)?;
        let val = obj.get("balance").ok_or("Can\'t get object from json.")?;
        <u64 as
            ::encoding::serialize::json::ExonumJson>::deserialize_field(val,
                                                                        buffer,
                                                                        from +
                                                                            40,
                                                                        from +
                                                                            48)?;
        Ok(())
    }
    #[allow(unused_mut)]
    fn serialize_field(&self)
     ->
         Result<::encoding::serialize::json::reexport::Value,
                Box<::std::error::Error>> {
        use ::encoding::serialize::json::reexport::Value;
        let mut map = ::encoding::serialize::json::reexport::Map::new();
        map.insert("pub_key".to_string(), self.pub_key().serialize_field()?);
        map.insert("name".to_string(), self.name().serialize_field()?);
        map.insert("balance".to_string(), self.balance().serialize_field()?);
        Ok(Value::Object(map))
    }
}
impl ::encoding::serialize::json::ExonumJsonDeserialize for Wallet {
    fn deserialize(value: &::encoding::serialize::json::reexport::Value)
     -> Result<Self, Box<::std::error::Error>> {
        let to = 48 as ::encoding::Offset;
        let from = 0;
        use ::encoding::serialize::json::ExonumJson;
        let mut buf = ::vec::from_elem(0, 48);
        <Self as ExonumJson>::deserialize_field(value, &mut buf, from, to)?;
        Ok(Wallet{raw: buf,})
    }
}
impl <'de> ::encoding::serialize::reexport::Deserialize<'de> for Wallet {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where
     D: ::encoding::serialize::reexport::Deserializer<'de> {
        use ::encoding::serialize::json::reexport::Value;
        use ::encoding::serialize::reexport::{DeError, Deserialize};
        let value = <Value as Deserialize>::deserialize(deserializer)?;
        <Self as
            ::encoding::serialize::json::ExonumJsonDeserialize>::deserialize(&value).map_err(|_|
                                                                                                 D::Error::custom("Can not deserialize value."))
    }
}
impl ::encoding::serialize::reexport::Serialize for Wallet {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where
     S: ::encoding::serialize::reexport::Serializer {
        use ::encoding::serialize::reexport::SerError;
        use ::encoding::serialize::json::ExonumJson;
        self.serialize_field().map_err(|_|
                                           S::Error::custom("Can not serialize structure: Wallet"))?.serialize(serializer)
    }
}
impl Wallet {
    pub fn set_balance(&mut self, value: u64) {
        Field::write(&value, &mut self.raw, 40, 48);
    }
}
impl SchemaRecordProvider for Wallet {
    fn get_schema() -> Option<SchemaRecord> {
        Some(SchemaRecord{name: ("Wallet").to_string(),
                          url:
                              ("http://localhost:8000/api/services/wallet/v1/schema").to_string(),
                          id: 0,
                          fields:
                              Some(<[_]>::into_vec(box
                                                       [SchemaField{name:
                                                                        ("pub_key").to_string(),
                                                                    ty:
                                                                        ("&PublicKey").to_string(),
                                                                    size: 32,
                                                                    modificators:
                                                                        None,},
                                                        SchemaField{name:
                                                                        ("name").to_string(),
                                                                    ty:
                                                                        ("&str").to_string(),
                                                                    size: 8,
                                                                    modificators:
                                                                        None,},
                                                        SchemaField{name:
                                                                        ("balance").to_string(),
                                                                    ty:
                                                                        ("u64").to_string(),
                                                                    size: 8,
                                                                    modificators:
                                                                        None,}])),
                          inputs: None,
                          outputs: None,})
    }
}
pub struct WalletSchema<'schema> {
    view: &'schema mut Fork,
}
impl <'schema> WalletSchema<'schema> {
    pub fn index(&mut self) -> MapIndex<&mut Fork, PublicKey, Wallet> {
        let prefix = blockchain::gen_prefix(1, 0, &());
        MapIndex::new(prefix, self.view)
    }
    pub fn get(&mut self, key: &PublicKey) -> Option<Wallet> {
        self.index().get(key)
    }
}
struct WalletApi {
    channel: ApiSender<NodeChannel>,
    schema: SchemaRecord,
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::clone::Clone for WalletApi {
    #[inline]
    fn clone(&self) -> WalletApi {
        match *self {
            WalletApi { channel: ref __self_0_0, schema: ref __self_0_1 } =>
            WalletApi{channel: ::std::clone::Clone::clone(&(*__self_0_0)),
                      schema: ::std::clone::Clone::clone(&(*__self_0_1)),},
        }
    }
}
impl Api for WalletApi {
    fn wire(&self, router: &mut Router) {
        #[serde(untagged)]
        enum TransactionRequest {
            create_wallet(TxCreateWallet),
            transfer(TxTransfer),
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::clone::Clone for TransactionRequest {
            #[inline]
            fn clone(&self) -> TransactionRequest {
                match (&*self,) {
                    (&TransactionRequest::create_wallet(ref __self_0),) =>
                    TransactionRequest::create_wallet(::std::clone::Clone::clone(&(*__self_0))),
                    (&TransactionRequest::transfer(ref __self_0),) =>
                    TransactionRequest::transfer(::std::clone::Clone::clone(&(*__self_0))),
                }
            }
        }
        #[allow(non_upper_case_globals,
                unused_attributes,
                unused_qualifications)]
        const _IMPL_SERIALIZE_FOR_TransactionRequest: () =
            {
                extern crate serde as _serde;
                #[automatically_derived]
                impl _serde::Serialize for TransactionRequest {
                    fn serialize<__S>(&self, __serializer: __S)
                     -> _serde::export::Result<__S::Ok, __S::Error> where
                     __S: _serde::Serializer {
                        match *self {
                            TransactionRequest::create_wallet(ref __field0) =>
                            _serde::Serialize::serialize(__field0,
                                                         __serializer),
                            TransactionRequest::transfer(ref __field0) =>
                            _serde::Serialize::serialize(__field0,
                                                         __serializer),
                        }
                    }
                }
            };
        #[allow(non_upper_case_globals,
                unused_attributes,
                unused_qualifications)]
        const _IMPL_DESERIALIZE_FOR_TransactionRequest: () =
            {
                extern crate serde as _serde;
                #[automatically_derived]
                impl <'de> _serde::Deserialize<'de> for TransactionRequest {
                    fn deserialize<__D>(__deserializer: __D)
                     -> _serde::export::Result<Self, __D::Error> where
                     __D: _serde::Deserializer<'de> {
                        let __content =
                            match <_serde::private::de::Content as
                                      _serde::Deserialize>::deserialize(__deserializer)
                                {
                                ::result::Result::Ok(val) => val,
                                ::result::Result::Err(err) => {
                                    return ::result::Result::Err(::convert::From::from(err))
                                }
                            };
                        if let _serde::export::Ok(__ok) =
                               _serde::export::Result::map(<TxCreateWallet as
                                                               _serde::Deserialize>::deserialize(_serde::private::de::ContentRefDeserializer::<__D::Error>::new(&__content)),
                                                           TransactionRequest::create_wallet)
                               {
                            return _serde::export::Ok(__ok);
                        }
                        if let _serde::export::Ok(__ok) =
                               _serde::export::Result::map(<TxTransfer as
                                                               _serde::Deserialize>::deserialize(_serde::private::de::ContentRefDeserializer::<__D::Error>::new(&__content)),
                                                           TransactionRequest::transfer)
                               {
                            return _serde::export::Ok(__ok);
                        }
                        _serde::export::Err(_serde::de::Error::custom("data did not match any variant of untagged enum TransactionRequest"))
                    }
                }
            };
        impl Into<Box<Transaction>> for TransactionRequest {
            fn into(self) -> Box<Transaction> {
                match self {
                    TransactionRequest::create_wallet(trans) =>
                    Box::new(trans),
                    TransactionRequest::transfer(trans) => Box::new(trans),
                }
            }
        }
        struct TransactionResponse {
            tx_hash: Hash,
        }
        #[allow(non_upper_case_globals,
                unused_attributes,
                unused_qualifications)]
        const _IMPL_SERIALIZE_FOR_TransactionResponse: () =
            {
                extern crate serde as _serde;
                #[automatically_derived]
                impl _serde::Serialize for TransactionResponse {
                    fn serialize<__S>(&self, __serializer: __S)
                     -> _serde::export::Result<__S::Ok, __S::Error> where
                     __S: _serde::Serializer {
                        let mut __serde_state =
                            match _serde::Serializer::serialize_struct(__serializer,
                                                                       "TransactionResponse",
                                                                       0 + 1)
                                {
                                ::result::Result::Ok(val) => val,
                                ::result::Result::Err(err) => {
                                    return ::result::Result::Err(::convert::From::from(err))
                                }
                            };
                        match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state,
                                                                            "tx_hash",
                                                                            &self.tx_hash)
                            {
                            ::result::Result::Ok(val) => val,
                            ::result::Result::Err(err) => {
                                return ::result::Result::Err(::convert::From::from(err))
                            }
                        };
                        _serde::ser::SerializeStruct::end(__serde_state)
                    }
                }
            };
        #[allow(non_upper_case_globals,
                unused_attributes,
                unused_qualifications)]
        const _IMPL_DESERIALIZE_FOR_TransactionResponse: () =
            {
                extern crate serde as _serde;
                #[automatically_derived]
                impl <'de> _serde::Deserialize<'de> for TransactionResponse {
                    fn deserialize<__D>(__deserializer: __D)
                     -> _serde::export::Result<Self, __D::Error> where
                     __D: _serde::Deserializer<'de> {
                        #[allow(non_camel_case_types)]
                        enum __Field { __field0, __ignore, }
                        struct __FieldVisitor;
                        impl <'de> _serde::de::Visitor<'de> for __FieldVisitor
                         {
                            type
                            Value
                            =
                            __Field;
                            fn expecting(&self,
                                         formatter:
                                             &mut _serde::export::Formatter)
                             -> _serde::export::fmt::Result {
                                _serde::export::Formatter::write_str(formatter,
                                                                     "field identifier")
                            }
                            fn visit_str<__E>(self, __value: &str)
                             -> _serde::export::Result<Self::Value, __E> where
                             __E: _serde::de::Error {
                                match __value {
                                    "tx_hash" =>
                                    _serde::export::Ok(__Field::__field0),
                                    _ =>
                                    _serde::export::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_bytes<__E>(self, __value: &[u8])
                             -> _serde::export::Result<Self::Value, __E> where
                             __E: _serde::de::Error {
                                match __value {
                                    b"tx_hash" =>
                                    _serde::export::Ok(__Field::__field0),
                                    _ => {
                                        _serde::export::Ok(__Field::__ignore)
                                    }
                                }
                            }
                        }
                        impl <'de> _serde::Deserialize<'de> for __Field {
                            #[inline]
                            fn deserialize<__D>(__deserializer: __D)
                             -> _serde::export::Result<Self, __D::Error> where
                             __D: _serde::Deserializer<'de> {
                                _serde::Deserializer::deserialize_identifier(__deserializer,
                                                                             __FieldVisitor)
                            }
                        }
                        struct __Visitor<'de> {
                            marker: _serde::export::PhantomData<TransactionResponse>,
                            lifetime: _serde::export::PhantomData<&'de ()>,
                        }
                        impl <'de> _serde::de::Visitor<'de> for __Visitor<'de>
                         {
                            type
                            Value
                            =
                            TransactionResponse;
                            fn expecting(&self,
                                         formatter:
                                             &mut _serde::export::Formatter)
                             -> _serde::export::fmt::Result {
                                _serde::export::Formatter::write_str(formatter,
                                                                     "struct TransactionResponse")
                            }
                            #[inline]
                            fn visit_seq<__A>(self, mut __seq: __A)
                             ->
                                 _serde::export::Result<Self::Value,
                                                        __A::Error> where
                             __A: _serde::de::SeqAccess<'de> {
                                let __field0 =
                                    match match _serde::de::SeqAccess::next_element::<Hash>(&mut __seq)
                                              {
                                              ::result::Result::Ok(val) =>
                                              val,
                                              ::result::Result::Err(err) => {
                                                  return ::result::Result::Err(::convert::From::from(err))
                                              }
                                          } {
                                        Some(__value) => __value,
                                        None => {
                                            return _serde::export::Err(_serde::de::Error::invalid_length(0usize,
                                                                                                         &"tuple of 1 elements"));
                                        }
                                    };
                                _serde::export::Ok(TransactionResponse{tx_hash:
                                                                           __field0,})
                            }
                            #[inline]
                            fn visit_map<__A>(self, mut __map: __A)
                             ->
                                 _serde::export::Result<Self::Value,
                                                        __A::Error> where
                             __A: _serde::de::MapAccess<'de> {
                                let mut __field0:
                                        _serde::export::Option<Hash> =
                                    _serde::export::None;
                                while let _serde::export::Some(__key) =
                                          match _serde::de::MapAccess::next_key::<__Field>(&mut __map)
                                              {
                                              ::result::Result::Ok(val) =>
                                              val,
                                              ::result::Result::Err(err) => {
                                                  return ::result::Result::Err(::convert::From::from(err))
                                              }
                                          } {
                                    match __key {
                                        __Field::__field0 => {
                                            if _serde::export::Option::is_some(&__field0)
                                               {
                                                return _serde::export::Err(<__A::Error
                                                                               as
                                                                               _serde::de::Error>::duplicate_field("tx_hash"));
                                            }
                                            __field0 =
                                                _serde::export::Some(match _serde::de::MapAccess::next_value::<Hash>(&mut __map)
                                                                         {
                                                                         ::result::Result::Ok(val)
                                                                         =>
                                                                         val,
                                                                         ::result::Result::Err(err)
                                                                         => {
                                                                             return ::result::Result::Err(::convert::From::from(err))
                                                                         }
                                                                     });
                                        }
                                        _ => {
                                            let _ =
                                                match _serde::de::MapAccess::next_value::<_serde::de::IgnoredAny>(&mut __map)
                                                    {
                                                    ::result::Result::Ok(val)
                                                    => val,
                                                    ::result::Result::Err(err)
                                                    => {
                                                        return ::result::Result::Err(::convert::From::from(err))
                                                    }
                                                };
                                        }
                                    }
                                }
                                let __field0 =
                                    match __field0 {
                                        _serde::export::Some(__field0) =>
                                        __field0,
                                        _serde::export::None =>
                                        match _serde::private::de::missing_field("tx_hash")
                                            {
                                            ::result::Result::Ok(val) => val,
                                            ::result::Result::Err(err) => {
                                                return ::result::Result::Err(::convert::From::from(err))
                                            }
                                        },
                                    };
                                _serde::export::Ok(TransactionResponse{tx_hash:
                                                                           __field0,})
                            }
                        }
                        const FIELDS: &'static [&'static str] = &["tx_hash"];
                        _serde::Deserializer::deserialize_struct(__deserializer,
                                                                 "TransactionResponse",
                                                                 FIELDS,
                                                                 __Visitor{marker:
                                                                               _serde::export::PhantomData::<TransactionResponse>,
                                                                           lifetime:
                                                                               _serde::export::PhantomData,})
                    }
                }
            };
        let self_ = self.clone();
        let transaction =
            move |req: &mut Request| -> IronResult<Response>
                {
                    match req.get::<bodyparser::Struct<TransactionRequest>>()
                        {
                        Ok(Some(transaction)) => {
                            let transaction: Box<Transaction> =
                                transaction.into();
                            let tx_hash = transaction.hash();
                            self_.channel.send(transaction).map_err(|e|
                                                                        ApiError::Events(e))?;
                            let json = TransactionResponse{tx_hash,};
                            self_.ok_response(&serde_json::to_value(&json).unwrap())
                        }
                        Ok(None) =>
                        Err(ApiError::IncorrectRequest("Empty request body".into()))?,
                        Err(e) =>
                        Err(ApiError::IncorrectRequest(Box::new(e)))?,
                    }
                };
        let self_ = self.clone();
        let schema_handler =
            move |red: &mut Request| -> IronResult<Response>
                {
                    self_.ok_response(&serde_json::to_value(&self_.schema).unwrap())
                };
        let route_post = "/v1/tx";
        router.post(&route_post, transaction, "transaction");
        let route_schema = "/v1/schema";
        router.get(&route_schema, schema_handler, "schema");
    }
}
struct WalletService;
impl Service for WalletService {
    fn service_name(&self) -> &'static str { "wallet" }
    fn service_id(&self) -> u16 { 1 }
    fn tx_from_raw(&self, raw: RawTransaction)
     -> Result<Box<Transaction>, encoding::Error> {
        let trans: Box<Transaction> =
            match raw.message_type() {
                1 => Box::new(TxCreateWallet::from_raw(raw)?),
                2 => Box::new(TxTransfer::from_raw(raw)?),
                _ => {
                    return Err(encoding::Error::IncorrectMessageType{message_type:
                                                                         raw.message_type(),});
                }
            };
        Ok(trans)
    }
    fn public_api_handler(&self, ctx: &ApiContext) -> Option<Box<Handler>> {
        let mut router = Router::new();
        let api =
            WalletApi{channel: ctx.node_channel().clone(),
                      schema: Wallet::get_schema().unwrap(),};
        api.wire(&mut router);
        Some(Box::new(router))
    }
}
impl Wallet {
    pub fn increase(&mut self, amount: u64) {
        let balance = self.balance() + amount;
        self.set_balance(balance);
    }
    pub fn decrease(&mut self, amount: u64) {
        let balance = self.balance() - amount;
        self.set_balance(balance);
    }
}
#[api = "WalletApi"]
#[service = "1"]
#[id = "1"]
struct __CreateWallet {
    #[key]
    #[size = "32"]
    pub_key: &'static PublicKey,
    #[size = "8"]
    name: &'static str,
}
pub struct TxCreateWallet {
    raw: ::messages::RawMessage,
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::clone::Clone for TxCreateWallet {
    #[inline]
    fn clone(&self) -> TxCreateWallet {
        match *self {
            TxCreateWallet { raw: ref __self_0_0 } =>
            TxCreateWallet{raw: ::std::clone::Clone::clone(&(*__self_0_0)),},
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::cmp::PartialEq for TxCreateWallet {
    #[inline]
    fn eq(&self, __arg_0: &TxCreateWallet) -> bool {
        match *__arg_0 {
            TxCreateWallet { raw: ref __self_1_0 } =>
            match *self {
                TxCreateWallet { raw: ref __self_0_0 } =>
                true && (*__self_0_0) == (*__self_1_0),
            },
        }
    }
    #[inline]
    fn ne(&self, __arg_0: &TxCreateWallet) -> bool {
        match *__arg_0 {
            TxCreateWallet { raw: ref __self_1_0 } =>
            match *self {
                TxCreateWallet { raw: ref __self_0_0 } =>
                false || (*__self_0_0) != (*__self_1_0),
            },
        }
    }
}
impl ::messages::Message for TxCreateWallet {
    fn raw(&self) -> &::messages::RawMessage { &self.raw }
}
impl <'a> ::encoding::SegmentField<'a> for TxCreateWallet {
    fn item_size() -> ::encoding::Offset { 1 }
    fn count(&self) -> ::encoding::Offset {
        self.raw.len() as ::encoding::Offset
    }
    fn extend_buffer(&self, buffer: &mut Vec<u8>) {
        buffer.extend_from_slice(self.raw.as_ref().as_ref())
    }
    unsafe fn from_buffer(buffer: &'a [u8], from: ::encoding::Offset,
                          count: ::encoding::Offset) -> Self {
        let raw_message: ::messages::RawMessage =
            ::encoding::SegmentField::from_buffer(buffer, from, count);
        ::messages::FromRaw::from_raw(raw_message).unwrap()
    }
    fn check_data(buffer: &'a [u8], from: ::encoding::CheckedOffset,
                  count: ::encoding::CheckedOffset,
                  latest_segment: ::encoding::CheckedOffset)
     -> ::encoding::Result {
        let latest_segment_origin =
            <::messages::RawMessage as
                ::encoding::SegmentField>::check_data(buffer, from, count,
                                                      latest_segment)?;
        let raw_message: ::messages::RawMessage =
            unsafe {
                ::encoding::SegmentField::from_buffer(buffer,
                                                      from.unchecked_offset(),
                                                      count.unchecked_offset())
            };
        let _: TxCreateWallet = ::messages::FromRaw::from_raw(raw_message)?;
        Ok(latest_segment_origin)
    }
}
impl ::messages::FromRaw for TxCreateWallet {
    fn from_raw(raw: ::messages::RawMessage)
     -> Result<TxCreateWallet, ::encoding::Error> {
        if raw.len() < (40 as usize) {
            return Err(::encoding::Error::UnexpectedlyShortPayload{actual_size:
                                                                       raw.len()
                                                                           as
                                                                           ::encoding::Offset,
                                                                   minimum_size:
                                                                       40,});
        }
        let len = <Self>::check_fields(&raw)?;
        if (len.unchecked_offset() as usize) +
               (::crypto::SIGNATURE_LENGTH as usize) != raw.len() {
            return Err("Incorrect raw message length.".into())
        }
        Ok(TxCreateWallet{raw: raw,})
    }
}
impl TxCreateWallet {
    /// Creates messsage and sign it.
    #[allow(unused_mut)]
    pub fn new(pub_key: &PublicKey, name: &str,
               secret_key: &::crypto::SecretKey) -> TxCreateWallet {
        use ::messages::{RawMessage, MessageWriter};
        {
            use ::encoding::Field;
            if true {
                {
                    match (&(0), &(0)) {
                        (left_val, right_val) => {
                            if !(*left_val == *right_val) {
                                {
                                    ::rt::begin_panic_fmt(&::std::fmt::Arguments::new_v1({
                                                                                             static __STATIC_FMTSTR:
                                                                                                    &'static [&'static str]
                                                                                                    =
                                                                                                 &["assertion failed: `(left == right)`\n  left: `",
                                                                                                   "`,\n right: `",
                                                                                                   "`: "];
                                                                                             __STATIC_FMTSTR
                                                                                         },
                                                                                         &match (&left_val,
                                                                                                 &right_val,
                                                                                                 &::std::fmt::Arguments::new_v1({
                                                                                                                                    static __STATIC_FMTSTR:
                                                                                                                                           &'static [&'static str]
                                                                                                                                           =
                                                                                                                                        &["first field should start from 0"];
                                                                                                                                    __STATIC_FMTSTR
                                                                                                                                },
                                                                                                                                &match ()
                                                                                                                                     {
                                                                                                                                     ()
                                                                                                                                     =>
                                                                                                                                     [],
                                                                                                                                 }))
                                                                                              {
                                                                                              (__arg0,
                                                                                               __arg1,
                                                                                               __arg2)
                                                                                              =>
                                                                                              [::std::fmt::ArgumentV1::new(__arg0,
                                                                                                                           ::std::fmt::Debug::fmt),
                                                                                               ::std::fmt::ArgumentV1::new(__arg1,
                                                                                                                           ::std::fmt::Debug::fmt),
                                                                                               ::std::fmt::ArgumentV1::new(__arg2,
                                                                                                                           ::std::fmt::Display::fmt)],
                                                                                          }),
                                                          {
                                                              static _FILE_LINE_COL:
                                                                     (&'static str,
                                                                      u32,
                                                                      u32) =
                                                                  ("src/main.rs",
                                                                   68u32,
                                                                   9u32);
                                                              &_FILE_LINE_COL
                                                          })
                                }
                            }
                        }
                    }
                };
            };
            if true {
                {
                    match (&(32 - 0), &(<&PublicKey as Field>::field_size()))
                        {
                        (left_val, right_val) => {
                            if !(*left_val == *right_val) {
                                {
                                    ::rt::begin_panic_fmt(&::std::fmt::Arguments::new_v1({
                                                                                             static __STATIC_FMTSTR:
                                                                                                    &'static [&'static str]
                                                                                                    =
                                                                                                 &["assertion failed: `(left == right)`\n  left: `",
                                                                                                   "`,\n right: `",
                                                                                                   "`: "];
                                                                                             __STATIC_FMTSTR
                                                                                         },
                                                                                         &match (&left_val,
                                                                                                 &right_val,
                                                                                                 &::std::fmt::Arguments::new_v1({
                                                                                                                                    static __STATIC_FMTSTR:
                                                                                                                                           &'static [&'static str]
                                                                                                                                           =
                                                                                                                                        &["wrong size of field"];
                                                                                                                                    __STATIC_FMTSTR
                                                                                                                                },
                                                                                                                                &match ()
                                                                                                                                     {
                                                                                                                                     ()
                                                                                                                                     =>
                                                                                                                                     [],
                                                                                                                                 }))
                                                                                              {
                                                                                              (__arg0,
                                                                                               __arg1,
                                                                                               __arg2)
                                                                                              =>
                                                                                              [::std::fmt::ArgumentV1::new(__arg0,
                                                                                                                           ::std::fmt::Debug::fmt),
                                                                                               ::std::fmt::ArgumentV1::new(__arg1,
                                                                                                                           ::std::fmt::Debug::fmt),
                                                                                               ::std::fmt::ArgumentV1::new(__arg2,
                                                                                                                           ::std::fmt::Display::fmt)],
                                                                                          }),
                                                          {
                                                              static _FILE_LINE_COL:
                                                                     (&'static str,
                                                                      u32,
                                                                      u32) =
                                                                  ("src/main.rs",
                                                                   68u32,
                                                                   9u32);
                                                              &_FILE_LINE_COL
                                                          })
                                }
                            }
                        }
                    }
                };
            };
            if true {
                {
                    match (&(32), &(32)) {
                        (left_val, right_val) => {
                            if !(*left_val == *right_val) {
                                {
                                    ::rt::begin_panic_fmt(&::std::fmt::Arguments::new_v1({
                                                                                             static __STATIC_FMTSTR:
                                                                                                    &'static [&'static str]
                                                                                                    =
                                                                                                 &["assertion failed: `(left == right)`\n  left: `",
                                                                                                   "`,\n right: `",
                                                                                                   "`: "];
                                                                                             __STATIC_FMTSTR
                                                                                         },
                                                                                         &match (&left_val,
                                                                                                 &right_val,
                                                                                                 &::std::fmt::Arguments::new_v1({
                                                                                                                                    static __STATIC_FMTSTR:
                                                                                                                                           &'static [&'static str]
                                                                                                                                           =
                                                                                                                                        &["fields should be adjacent"];
                                                                                                                                    __STATIC_FMTSTR
                                                                                                                                },
                                                                                                                                &match ()
                                                                                                                                     {
                                                                                                                                     ()
                                                                                                                                     =>
                                                                                                                                     [],
                                                                                                                                 }))
                                                                                              {
                                                                                              (__arg0,
                                                                                               __arg1,
                                                                                               __arg2)
                                                                                              =>
                                                                                              [::std::fmt::ArgumentV1::new(__arg0,
                                                                                                                           ::std::fmt::Debug::fmt),
                                                                                               ::std::fmt::ArgumentV1::new(__arg1,
                                                                                                                           ::std::fmt::Debug::fmt),
                                                                                               ::std::fmt::ArgumentV1::new(__arg2,
                                                                                                                           ::std::fmt::Display::fmt)],
                                                                                          }),
                                                          {
                                                              static _FILE_LINE_COL:
                                                                     (&'static str,
                                                                      u32,
                                                                      u32) =
                                                                  ("src/main.rs",
                                                                   68u32,
                                                                   9u32);
                                                              &_FILE_LINE_COL
                                                          })
                                }
                            }
                        }
                    }
                };
            };
            if true {
                {
                    match (&(40), &(40)) {
                        (left_val, right_val) => {
                            if !(*left_val == *right_val) {
                                {
                                    ::rt::begin_panic_fmt(&::std::fmt::Arguments::new_v1({
                                                                                             static __STATIC_FMTSTR:
                                                                                                    &'static [&'static str]
                                                                                                    =
                                                                                                 &["assertion failed: `(left == right)`\n  left: `",
                                                                                                   "`,\n right: `",
                                                                                                   "`: "];
                                                                                             __STATIC_FMTSTR
                                                                                         },
                                                                                         &match (&left_val,
                                                                                                 &right_val,
                                                                                                 &::std::fmt::Arguments::new_v1({
                                                                                                                                    static __STATIC_FMTSTR:
                                                                                                                                           &'static [&'static str]
                                                                                                                                           =
                                                                                                                                        &["last field should matches the size of struct"];
                                                                                                                                    __STATIC_FMTSTR
                                                                                                                                },
                                                                                                                                &match ()
                                                                                                                                     {
                                                                                                                                     ()
                                                                                                                                     =>
                                                                                                                                     [],
                                                                                                                                 }))
                                                                                              {
                                                                                              (__arg0,
                                                                                               __arg1,
                                                                                               __arg2)
                                                                                              =>
                                                                                              [::std::fmt::ArgumentV1::new(__arg0,
                                                                                                                           ::std::fmt::Debug::fmt),
                                                                                               ::std::fmt::ArgumentV1::new(__arg1,
                                                                                                                           ::std::fmt::Debug::fmt),
                                                                                               ::std::fmt::ArgumentV1::new(__arg2,
                                                                                                                           ::std::fmt::Display::fmt)],
                                                                                          }),
                                                          {
                                                              static _FILE_LINE_COL:
                                                                     (&'static str,
                                                                      u32,
                                                                      u32) =
                                                                  ("src/main.rs",
                                                                   68u32,
                                                                   9u32);
                                                              &_FILE_LINE_COL
                                                          })
                                }
                            }
                        }
                    }
                };
            };
            if true {
                {
                    match (&(40 - 32), &(<&str as Field>::field_size())) {
                        (left_val, right_val) => {
                            if !(*left_val == *right_val) {
                                {
                                    ::rt::begin_panic_fmt(&::std::fmt::Arguments::new_v1({
                                                                                             static __STATIC_FMTSTR:
                                                                                                    &'static [&'static str]
                                                                                                    =
                                                                                                 &["assertion failed: `(left == right)`\n  left: `",
                                                                                                   "`,\n right: `",
                                                                                                   "`: "];
                                                                                             __STATIC_FMTSTR
                                                                                         },
                                                                                         &match (&left_val,
                                                                                                 &right_val,
                                                                                                 &::std::fmt::Arguments::new_v1({
                                                                                                                                    static __STATIC_FMTSTR:
                                                                                                                                           &'static [&'static str]
                                                                                                                                           =
                                                                                                                                        &["wrong size of field"];
                                                                                                                                    __STATIC_FMTSTR
                                                                                                                                },
                                                                                                                                &match ()
                                                                                                                                     {
                                                                                                                                     ()
                                                                                                                                     =>
                                                                                                                                     [],
                                                                                                                                 }))
                                                                                              {
                                                                                              (__arg0,
                                                                                               __arg1,
                                                                                               __arg2)
                                                                                              =>
                                                                                              [::std::fmt::ArgumentV1::new(__arg0,
                                                                                                                           ::std::fmt::Debug::fmt),
                                                                                               ::std::fmt::ArgumentV1::new(__arg1,
                                                                                                                           ::std::fmt::Debug::fmt),
                                                                                               ::std::fmt::ArgumentV1::new(__arg2,
                                                                                                                           ::std::fmt::Display::fmt)],
                                                                                          }),
                                                          {
                                                              static _FILE_LINE_COL:
                                                                     (&'static str,
                                                                      u32,
                                                                      u32) =
                                                                  ("src/main.rs",
                                                                   68u32,
                                                                   9u32);
                                                              &_FILE_LINE_COL
                                                          })
                                }
                            }
                        }
                    }
                };
            };
        };
        let mut writer =
            MessageWriter::new(::messages::PROTOCOL_MAJOR_VERSION,
                               ::messages::TEST_NETWORK_ID, 1, 1, 40);
        writer.write(pub_key, 0, 32);
        writer.write(name, 32, 40);
        TxCreateWallet{raw: RawMessage::new(writer.sign(secret_key)),}
    }
    /// Creates message and appends existing signature.
    #[allow(dead_code, unused_mut)]
    pub fn new_with_signature(pub_key: &PublicKey, name: &str,
                              signature: &::crypto::Signature)
     -> TxCreateWallet {
        use ::messages::{RawMessage, MessageWriter};
        {
            use ::encoding::Field;
            if true {
                {
                    match (&(0), &(0)) {
                        (left_val, right_val) => {
                            if !(*left_val == *right_val) {
                                {
                                    ::rt::begin_panic_fmt(&::std::fmt::Arguments::new_v1({
                                                                                             static __STATIC_FMTSTR:
                                                                                                    &'static [&'static str]
                                                                                                    =
                                                                                                 &["assertion failed: `(left == right)`\n  left: `",
                                                                                                   "`,\n right: `",
                                                                                                   "`: "];
                                                                                             __STATIC_FMTSTR
                                                                                         },
                                                                                         &match (&left_val,
                                                                                                 &right_val,
                                                                                                 &::std::fmt::Arguments::new_v1({
                                                                                                                                    static __STATIC_FMTSTR:
                                                                                                                                           &'static [&'static str]
                                                                                                                                           =
                                                                                                                                        &["first field should start from 0"];
                                                                                                                                    __STATIC_FMTSTR
                                                                                                                                },
                                                                                                                                &match ()
                                                                                                                                     {
                                                                                                                                     ()
                                                                                                                                     =>
                                                                                                                                     [],
                                                                                                                                 }))
                                                                                              {
                                                                                              (__arg0,
                                                                                               __arg1,
                                                                                               __arg2)
                                                                                              =>
                                                                                              [::std::fmt::ArgumentV1::new(__arg0,
                                                                                                                           ::std::fmt::Debug::fmt),
                                                                                               ::std::fmt::ArgumentV1::new(__arg1,
                                                                                                                           ::std::fmt::Debug::fmt),
                                                                                               ::std::fmt::ArgumentV1::new(__arg2,
                                                                                                                           ::std::fmt::Display::fmt)],
                                                                                          }),
                                                          {
                                                              static _FILE_LINE_COL:
                                                                     (&'static str,
                                                                      u32,
                                                                      u32) =
                                                                  ("src/main.rs",
                                                                   68u32,
                                                                   9u32);
                                                              &_FILE_LINE_COL
                                                          })
                                }
                            }
                        }
                    }
                };
            };
            if true {
                {
                    match (&(32 - 0), &(<&PublicKey as Field>::field_size()))
                        {
                        (left_val, right_val) => {
                            if !(*left_val == *right_val) {
                                {
                                    ::rt::begin_panic_fmt(&::std::fmt::Arguments::new_v1({
                                                                                             static __STATIC_FMTSTR:
                                                                                                    &'static [&'static str]
                                                                                                    =
                                                                                                 &["assertion failed: `(left == right)`\n  left: `",
                                                                                                   "`,\n right: `",
                                                                                                   "`: "];
                                                                                             __STATIC_FMTSTR
                                                                                         },
                                                                                         &match (&left_val,
                                                                                                 &right_val,
                                                                                                 &::std::fmt::Arguments::new_v1({
                                                                                                                                    static __STATIC_FMTSTR:
                                                                                                                                           &'static [&'static str]
                                                                                                                                           =
                                                                                                                                        &["wrong size of field"];
                                                                                                                                    __STATIC_FMTSTR
                                                                                                                                },
                                                                                                                                &match ()
                                                                                                                                     {
                                                                                                                                     ()
                                                                                                                                     =>
                                                                                                                                     [],
                                                                                                                                 }))
                                                                                              {
                                                                                              (__arg0,
                                                                                               __arg1,
                                                                                               __arg2)
                                                                                              =>
                                                                                              [::std::fmt::ArgumentV1::new(__arg0,
                                                                                                                           ::std::fmt::Debug::fmt),
                                                                                               ::std::fmt::ArgumentV1::new(__arg1,
                                                                                                                           ::std::fmt::Debug::fmt),
                                                                                               ::std::fmt::ArgumentV1::new(__arg2,
                                                                                                                           ::std::fmt::Display::fmt)],
                                                                                          }),
                                                          {
                                                              static _FILE_LINE_COL:
                                                                     (&'static str,
                                                                      u32,
                                                                      u32) =
                                                                  ("src/main.rs",
                                                                   68u32,
                                                                   9u32);
                                                              &_FILE_LINE_COL
                                                          })
                                }
                            }
                        }
                    }
                };
            };
            if true {
                {
                    match (&(32), &(32)) {
                        (left_val, right_val) => {
                            if !(*left_val == *right_val) {
                                {
                                    ::rt::begin_panic_fmt(&::std::fmt::Arguments::new_v1({
                                                                                             static __STATIC_FMTSTR:
                                                                                                    &'static [&'static str]
                                                                                                    =
                                                                                                 &["assertion failed: `(left == right)`\n  left: `",
                                                                                                   "`,\n right: `",
                                                                                                   "`: "];
                                                                                             __STATIC_FMTSTR
                                                                                         },
                                                                                         &match (&left_val,
                                                                                                 &right_val,
                                                                                                 &::std::fmt::Arguments::new_v1({
                                                                                                                                    static __STATIC_FMTSTR:
                                                                                                                                           &'static [&'static str]
                                                                                                                                           =
                                                                                                                                        &["fields should be adjacent"];
                                                                                                                                    __STATIC_FMTSTR
                                                                                                                                },
                                                                                                                                &match ()
                                                                                                                                     {
                                                                                                                                     ()
                                                                                                                                     =>
                                                                                                                                     [],
                                                                                                                                 }))
                                                                                              {
                                                                                              (__arg0,
                                                                                               __arg1,
                                                                                               __arg2)
                                                                                              =>
                                                                                              [::std::fmt::ArgumentV1::new(__arg0,
                                                                                                                           ::std::fmt::Debug::fmt),
                                                                                               ::std::fmt::ArgumentV1::new(__arg1,
                                                                                                                           ::std::fmt::Debug::fmt),
                                                                                               ::std::fmt::ArgumentV1::new(__arg2,
                                                                                                                           ::std::fmt::Display::fmt)],
                                                                                          }),
                                                          {
                                                              static _FILE_LINE_COL:
                                                                     (&'static str,
                                                                      u32,
                                                                      u32) =
                                                                  ("src/main.rs",
                                                                   68u32,
                                                                   9u32);
                                                              &_FILE_LINE_COL
                                                          })
                                }
                            }
                        }
                    }
                };
            };
            if true {
                {
                    match (&(40), &(40)) {
                        (left_val, right_val) => {
                            if !(*left_val == *right_val) {
                                {
                                    ::rt::begin_panic_fmt(&::std::fmt::Arguments::new_v1({
                                                                                             static __STATIC_FMTSTR:
                                                                                                    &'static [&'static str]
                                                                                                    =
                                                                                                 &["assertion failed: `(left == right)`\n  left: `",
                                                                                                   "`,\n right: `",
                                                                                                   "`: "];
                                                                                             __STATIC_FMTSTR
                                                                                         },
                                                                                         &match (&left_val,
                                                                                                 &right_val,
                                                                                                 &::std::fmt::Arguments::new_v1({
                                                                                                                                    static __STATIC_FMTSTR:
                                                                                                                                           &'static [&'static str]
                                                                                                                                           =
                                                                                                                                        &["last field should matches the size of struct"];
                                                                                                                                    __STATIC_FMTSTR
                                                                                                                                },
                                                                                                                                &match ()
                                                                                                                                     {
                                                                                                                                     ()
                                                                                                                                     =>
                                                                                                                                     [],
                                                                                                                                 }))
                                                                                              {
                                                                                              (__arg0,
                                                                                               __arg1,
                                                                                               __arg2)
                                                                                              =>
                                                                                              [::std::fmt::ArgumentV1::new(__arg0,
                                                                                                                           ::std::fmt::Debug::fmt),
                                                                                               ::std::fmt::ArgumentV1::new(__arg1,
                                                                                                                           ::std::fmt::Debug::fmt),
                                                                                               ::std::fmt::ArgumentV1::new(__arg2,
                                                                                                                           ::std::fmt::Display::fmt)],
                                                                                          }),
                                                          {
                                                              static _FILE_LINE_COL:
                                                                     (&'static str,
                                                                      u32,
                                                                      u32) =
                                                                  ("src/main.rs",
                                                                   68u32,
                                                                   9u32);
                                                              &_FILE_LINE_COL
                                                          })
                                }
                            }
                        }
                    }
                };
            };
            if true {
                {
                    match (&(40 - 32), &(<&str as Field>::field_size())) {
                        (left_val, right_val) => {
                            if !(*left_val == *right_val) {
                                {
                                    ::rt::begin_panic_fmt(&::std::fmt::Arguments::new_v1({
                                                                                             static __STATIC_FMTSTR:
                                                                                                    &'static [&'static str]
                                                                                                    =
                                                                                                 &["assertion failed: `(left == right)`\n  left: `",
                                                                                                   "`,\n right: `",
                                                                                                   "`: "];
                                                                                             __STATIC_FMTSTR
                                                                                         },
                                                                                         &match (&left_val,
                                                                                                 &right_val,
                                                                                                 &::std::fmt::Arguments::new_v1({
                                                                                                                                    static __STATIC_FMTSTR:
                                                                                                                                           &'static [&'static str]
                                                                                                                                           =
                                                                                                                                        &["wrong size of field"];
                                                                                                                                    __STATIC_FMTSTR
                                                                                                                                },
                                                                                                                                &match ()
                                                                                                                                     {
                                                                                                                                     ()
                                                                                                                                     =>
                                                                                                                                     [],
                                                                                                                                 }))
                                                                                              {
                                                                                              (__arg0,
                                                                                               __arg1,
                                                                                               __arg2)
                                                                                              =>
                                                                                              [::std::fmt::ArgumentV1::new(__arg0,
                                                                                                                           ::std::fmt::Debug::fmt),
                                                                                               ::std::fmt::ArgumentV1::new(__arg1,
                                                                                                                           ::std::fmt::Debug::fmt),
                                                                                               ::std::fmt::ArgumentV1::new(__arg2,
                                                                                                                           ::std::fmt::Display::fmt)],
                                                                                          }),
                                                          {
                                                              static _FILE_LINE_COL:
                                                                     (&'static str,
                                                                      u32,
                                                                      u32) =
                                                                  ("src/main.rs",
                                                                   68u32,
                                                                   9u32);
                                                              &_FILE_LINE_COL
                                                          })
                                }
                            }
                        }
                    }
                };
            };
        };
        let mut writer =
            MessageWriter::new(::messages::PROTOCOL_MAJOR_VERSION,
                               ::messages::TEST_NETWORK_ID, 1, 1, 40);
        writer.write(pub_key, 0, 32);
        writer.write(name, 32, 40);
        TxCreateWallet{raw:
                           RawMessage::new(writer.append_signature(signature)),}
    }
    #[allow(unused_variables)]
    fn check_fields(raw_message: &::messages::RawMessage)
     -> ::encoding::Result {
        let latest_segment =
            ((40 + ::messages::HEADER_LENGTH) as ::encoding::Offset).into();
        let field_from: ::encoding::Offset = 0;
        let field_to: ::encoding::Offset = 32;
        let latest_segment =
            raw_message.check::<&PublicKey>(field_from.into(),
                                            field_to.into(), latest_segment)?;
        let field_from: ::encoding::Offset = 32;
        let field_to: ::encoding::Offset = 40;
        let latest_segment =
            raw_message.check::<&str>(field_from.into(), field_to.into(),
                                      latest_segment)?;
        Ok(latest_segment)
    }
    /// Returns `message_id` useable for matching.
    #[allow(dead_code)]
    pub fn message_id() -> u16 { 1 }
    /// Returns `service_id` useable for matching.
    #[allow(dead_code)]
    pub fn service_id() -> u16 { 1 }
    pub fn pub_key(&self) -> &PublicKey {
        unsafe { self.raw.read::<&PublicKey>(0, 32) }
    }
    pub fn name(&self) -> &str { unsafe { self.raw.read::<&str>(32, 40) } }
}
impl ::storage::StorageValue for TxCreateWallet {
    fn hash(&self) -> ::crypto::Hash { ::messages::Message::hash(self) }
    fn into_bytes(self) -> Vec<u8> { self.raw.as_ref().as_ref().to_vec() }
    fn from_bytes(value: ::std::borrow::Cow<[u8]>) -> Self {
        TxCreateWallet{raw:
                           ::std::sync::Arc::new(::messages::MessageBuffer::from_vec(value.into_owned())),}
    }
}
impl ::std::fmt::Debug for TxCreateWallet {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter)
     -> Result<(), ::std::fmt::Error> {
        fmt.debug_struct("TxCreateWallet").field("pub_key",
                                                 &self.pub_key()).field("name",
                                                                        &self.name()).finish()
    }
}
impl ::encoding::serialize::json::ExonumJson for TxCreateWallet {
    fn deserialize_field<B>(value:
                                &::encoding::serialize::json::reexport::Value,
                            buffer: &mut B, from: ::encoding::Offset,
                            to: ::encoding::Offset)
     -> Result<(), Box<::std::error::Error>> where
     B: ::encoding::serialize::WriteBufferWrapper {
        use ::encoding::serialize::json::ExonumJsonDeserialize;
        let structure = <Self as ExonumJsonDeserialize>::deserialize(value)?;
        buffer.write(from, to, structure);
        Ok(())
    }
    #[allow(unused_mut)]
    fn serialize_field(&self)
     ->
         Result<::encoding::serialize::json::reexport::Value,
                Box<::std::error::Error>> {
        use ::encoding::serialize::json::reexport::Value;
        use ::encoding::serialize::json::reexport::Map;
        let mut body = Map::new();
        body.insert("pub_key".to_string(), self.pub_key().serialize_field()?);
        body.insert("name".to_string(), self.name().serialize_field()?);
        let mut structure = Map::new();
        structure.insert("body".to_string(), Value::Object(body));
        structure.insert("signature".to_string(),
                         self.raw.signature().serialize_field()?);
        structure.insert("message_id".to_string(),
                         self.raw.message_type().serialize_field()?);
        structure.insert("service_id".to_string(),
                         self.raw.service_id().serialize_field()?);
        structure.insert("network_id".to_string(),
                         self.raw.network_id().serialize_field()?);
        structure.insert("protocol_version".to_string(),
                         self.raw.version().serialize_field()?);
        Ok(Value::Object(structure))
    }
}
impl ::encoding::serialize::json::ExonumJsonDeserialize for TxCreateWallet {
    #[allow(unused_imports, unused_variables, unused_mut)]
    fn deserialize(value: &::encoding::serialize::json::reexport::Value)
     -> Result<Self, Box<::std::error::Error>> {
        use ::encoding::serialize::json::ExonumJson;
        use ::encoding::serialize::json::reexport::from_value;
        use ::messages::{RawMessage, MessageWriter};
        let obj = value.as_object().ok_or("Can\'t cast json as object.")?;
        let body = obj.get("body").ok_or("Can\'t get body from json.")?;
        let signature =
            from_value(obj.get("signature").ok_or("Can\'t get signature from json")?.clone())?;
        let message_type =
            from_value(obj.get("message_id").ok_or("Can\'t get message_type from json")?.clone())?;
        let service_id =
            from_value(obj.get("service_id").ok_or("Can\'t get service_id from json")?.clone())?;
        let network_id =
            from_value(obj.get("network_id").ok_or("Can\'t get network_id from json")?.clone())?;
        let protocol_version =
            from_value(obj.get("protocol_version").ok_or("Can\'t get protocol_version from json")?.clone())?;
        if service_id != 1 {
            return Err("service_id didn\'t equal real service_id.".into())
        }
        if message_type != 1 {
            return Err("message_id didn\'t equal real message_id.".into())
        }
        let mut writer =
            MessageWriter::new(protocol_version, network_id, service_id,
                               message_type, 40);
        let obj = body.as_object().ok_or("Can\'t cast body as object.")?;
        let val = obj.get("pub_key").ok_or("Can\'t get object from json.")?;
        <&PublicKey as
            ExonumJson>::deserialize_field(val, &mut writer, 0, 32)?;
        let val = obj.get("name").ok_or("Can\'t get object from json.")?;
        <&str as ExonumJson>::deserialize_field(val, &mut writer, 32, 40)?;
        Ok(TxCreateWallet{raw:
                              RawMessage::new(writer.append_signature(&signature)),})
    }
}
impl <'de> ::encoding::serialize::reexport::Deserialize<'de> for
 TxCreateWallet {
    #[allow(unused_mut)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where
     D: ::encoding::serialize::reexport::Deserializer<'de> {
        use ::encoding::serialize::json::reexport::Value;
        use ::encoding::serialize::reexport::{DeError, Deserialize};
        let value = <Value as Deserialize>::deserialize(deserializer)?;
        <Self as
            ::encoding::serialize::json::ExonumJsonDeserialize>::deserialize(&value).map_err(|_|
                                                                                                 D::Error::custom("Can not deserialize value."))
    }
}
impl ::encoding::serialize::reexport::Serialize for TxCreateWallet {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where
     S: ::encoding::serialize::reexport::Serializer {
        use ::encoding::serialize::reexport::SerError;
        use ::encoding::serialize::json::ExonumJson;
        self.serialize_field().map_err(|_|
                                           S::Error::custom("Can not serialize structure: TxCreateWallet"))?.serialize(serializer)
    }
}
impl Transaction for TxCreateWallet {
    fn verify(&self) -> bool {
        self.verify_signature(self.pub_key()) &&
            TransactionVerify::verify(self)
    }
    fn execute(&self, view: &mut Fork) {
        TransactionExecute::execute(self, view);
    }
}
#[api = "WalletApi"]
#[service = "1"]
#[id = "2"]
struct __Transfer {
    #[key]
    #[size = "32"]
    from: &'static PublicKey,
    #[size = "32"]
    to: &'static PublicKey,
    #[size = "8"]
    amount: u64,
    #[size = "8"]
    seed: u64,
}
pub struct TxTransfer {
    raw: ::messages::RawMessage,
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::clone::Clone for TxTransfer {
    #[inline]
    fn clone(&self) -> TxTransfer {
        match *self {
            TxTransfer { raw: ref __self_0_0 } =>
            TxTransfer{raw: ::std::clone::Clone::clone(&(*__self_0_0)),},
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::cmp::PartialEq for TxTransfer {
    #[inline]
    fn eq(&self, __arg_0: &TxTransfer) -> bool {
        match *__arg_0 {
            TxTransfer { raw: ref __self_1_0 } =>
            match *self {
                TxTransfer { raw: ref __self_0_0 } =>
                true && (*__self_0_0) == (*__self_1_0),
            },
        }
    }
    #[inline]
    fn ne(&self, __arg_0: &TxTransfer) -> bool {
        match *__arg_0 {
            TxTransfer { raw: ref __self_1_0 } =>
            match *self {
                TxTransfer { raw: ref __self_0_0 } =>
                false || (*__self_0_0) != (*__self_1_0),
            },
        }
    }
}
impl ::messages::Message for TxTransfer {
    fn raw(&self) -> &::messages::RawMessage { &self.raw }
}
impl <'a> ::encoding::SegmentField<'a> for TxTransfer {
    fn item_size() -> ::encoding::Offset { 1 }
    fn count(&self) -> ::encoding::Offset {
        self.raw.len() as ::encoding::Offset
    }
    fn extend_buffer(&self, buffer: &mut Vec<u8>) {
        buffer.extend_from_slice(self.raw.as_ref().as_ref())
    }
    unsafe fn from_buffer(buffer: &'a [u8], from: ::encoding::Offset,
                          count: ::encoding::Offset) -> Self {
        let raw_message: ::messages::RawMessage =
            ::encoding::SegmentField::from_buffer(buffer, from, count);
        ::messages::FromRaw::from_raw(raw_message).unwrap()
    }
    fn check_data(buffer: &'a [u8], from: ::encoding::CheckedOffset,
                  count: ::encoding::CheckedOffset,
                  latest_segment: ::encoding::CheckedOffset)
     -> ::encoding::Result {
        let latest_segment_origin =
            <::messages::RawMessage as
                ::encoding::SegmentField>::check_data(buffer, from, count,
                                                      latest_segment)?;
        let raw_message: ::messages::RawMessage =
            unsafe {
                ::encoding::SegmentField::from_buffer(buffer,
                                                      from.unchecked_offset(),
                                                      count.unchecked_offset())
            };
        let _: TxTransfer = ::messages::FromRaw::from_raw(raw_message)?;
        Ok(latest_segment_origin)
    }
}
impl ::messages::FromRaw for TxTransfer {
    fn from_raw(raw: ::messages::RawMessage)
     -> Result<TxTransfer, ::encoding::Error> {
        if raw.len() < (80 as usize) {
            return Err(::encoding::Error::UnexpectedlyShortPayload{actual_size:
                                                                       raw.len()
                                                                           as
                                                                           ::encoding::Offset,
                                                                   minimum_size:
                                                                       80,});
        }
        let len = <Self>::check_fields(&raw)?;
        if (len.unchecked_offset() as usize) +
               (::crypto::SIGNATURE_LENGTH as usize) != raw.len() {
            return Err("Incorrect raw message length.".into())
        }
        Ok(TxTransfer{raw: raw,})
    }
}
impl TxTransfer {
    /// Creates messsage and sign it.
    #[allow(unused_mut)]
    pub fn new(from: &PublicKey, to: &PublicKey, amount: u64, seed: u64,
               secret_key: &::crypto::SecretKey) -> TxTransfer {
        use ::messages::{RawMessage, MessageWriter};
        {
            use ::encoding::Field;
            if true {
                {
                    match (&(0), &(0)) {
                        (left_val, right_val) => {
                            if !(*left_val == *right_val) {
                                {
                                    ::rt::begin_panic_fmt(&::std::fmt::Arguments::new_v1({
                                                                                             static __STATIC_FMTSTR:
                                                                                                    &'static [&'static str]
                                                                                                    =
                                                                                                 &["assertion failed: `(left == right)`\n  left: `",
                                                                                                   "`,\n right: `",
                                                                                                   "`: "];
                                                                                             __STATIC_FMTSTR
                                                                                         },
                                                                                         &match (&left_val,
                                                                                                 &right_val,
                                                                                                 &::std::fmt::Arguments::new_v1({
                                                                                                                                    static __STATIC_FMTSTR:
                                                                                                                                           &'static [&'static str]
                                                                                                                                           =
                                                                                                                                        &["first field should start from 0"];
                                                                                                                                    __STATIC_FMTSTR
                                                                                                                                },
                                                                                                                                &match ()
                                                                                                                                     {
                                                                                                                                     ()
                                                                                                                                     =>
                                                                                                                                     [],
                                                                                                                                 }))
                                                                                              {
                                                                                              (__arg0,
                                                                                               __arg1,
                                                                                               __arg2)
                                                                                              =>
                                                                                              [::std::fmt::ArgumentV1::new(__arg0,
                                                                                                                           ::std::fmt::Debug::fmt),
                                                                                               ::std::fmt::ArgumentV1::new(__arg1,
                                                                                                                           ::std::fmt::Debug::fmt),
                                                                                               ::std::fmt::ArgumentV1::new(__arg2,
                                                                                                                           ::std::fmt::Display::fmt)],
                                                                                          }),
                                                          {
                                                              static _FILE_LINE_COL:
                                                                     (&'static str,
                                                                      u32,
                                                                      u32) =
                                                                  ("src/main.rs",
                                                                   81u32,
                                                                   9u32);
                                                              &_FILE_LINE_COL
                                                          })
                                }
                            }
                        }
                    }
                };
            };
            if true {
                {
                    match (&(32 - 0), &(<&PublicKey as Field>::field_size()))
                        {
                        (left_val, right_val) => {
                            if !(*left_val == *right_val) {
                                {
                                    ::rt::begin_panic_fmt(&::std::fmt::Arguments::new_v1({
                                                                                             static __STATIC_FMTSTR:
                                                                                                    &'static [&'static str]
                                                                                                    =
                                                                                                 &["assertion failed: `(left == right)`\n  left: `",
                                                                                                   "`,\n right: `",
                                                                                                   "`: "];
                                                                                             __STATIC_FMTSTR
                                                                                         },
                                                                                         &match (&left_val,
                                                                                                 &right_val,
                                                                                                 &::std::fmt::Arguments::new_v1({
                                                                                                                                    static __STATIC_FMTSTR:
                                                                                                                                           &'static [&'static str]
                                                                                                                                           =
                                                                                                                                        &["wrong size of field"];
                                                                                                                                    __STATIC_FMTSTR
                                                                                                                                },
                                                                                                                                &match ()
                                                                                                                                     {
                                                                                                                                     ()
                                                                                                                                     =>
                                                                                                                                     [],
                                                                                                                                 }))
                                                                                              {
                                                                                              (__arg0,
                                                                                               __arg1,
                                                                                               __arg2)
                                                                                              =>
                                                                                              [::std::fmt::ArgumentV1::new(__arg0,
                                                                                                                           ::std::fmt::Debug::fmt),
                                                                                               ::std::fmt::ArgumentV1::new(__arg1,
                                                                                                                           ::std::fmt::Debug::fmt),
                                                                                               ::std::fmt::ArgumentV1::new(__arg2,
                                                                                                                           ::std::fmt::Display::fmt)],
                                                                                          }),
                                                          {
                                                              static _FILE_LINE_COL:
                                                                     (&'static str,
                                                                      u32,
                                                                      u32) =
                                                                  ("src/main.rs",
                                                                   81u32,
                                                                   9u32);
                                                              &_FILE_LINE_COL
                                                          })
                                }
                            }
                        }
                    }
                };
            };
            if true {
                {
                    match (&(32), &(32)) {
                        (left_val, right_val) => {
                            if !(*left_val == *right_val) {
                                {
                                    ::rt::begin_panic_fmt(&::std::fmt::Arguments::new_v1({
                                                                                             static __STATIC_FMTSTR:
                                                                                                    &'static [&'static str]
                                                                                                    =
                                                                                                 &["assertion failed: `(left == right)`\n  left: `",
                                                                                                   "`,\n right: `",
                                                                                                   "`: "];
                                                                                             __STATIC_FMTSTR
                                                                                         },
                                                                                         &match (&left_val,
                                                                                                 &right_val,
                                                                                                 &::std::fmt::Arguments::new_v1({
                                                                                                                                    static __STATIC_FMTSTR:
                                                                                                                                           &'static [&'static str]
                                                                                                                                           =
                                                                                                                                        &["fields should be adjacent"];
                                                                                                                                    __STATIC_FMTSTR
                                                                                                                                },
                                                                                                                                &match ()
                                                                                                                                     {
                                                                                                                                     ()
                                                                                                                                     =>
                                                                                                                                     [],
                                                                                                                                 }))
                                                                                              {
                                                                                              (__arg0,
                                                                                               __arg1,
                                                                                               __arg2)
                                                                                              =>
                                                                                              [::std::fmt::ArgumentV1::new(__arg0,
                                                                                                                           ::std::fmt::Debug::fmt),
                                                                                               ::std::fmt::ArgumentV1::new(__arg1,
                                                                                                                           ::std::fmt::Debug::fmt),
                                                                                               ::std::fmt::ArgumentV1::new(__arg2,
                                                                                                                           ::std::fmt::Display::fmt)],
                                                                                          }),
                                                          {
                                                              static _FILE_LINE_COL:
                                                                     (&'static str,
                                                                      u32,
                                                                      u32) =
                                                                  ("src/main.rs",
                                                                   81u32,
                                                                   9u32);
                                                              &_FILE_LINE_COL
                                                          })
                                }
                            }
                        }
                    }
                };
            };
            if true {
                {
                    match (&(64 - 32), &(<&PublicKey as Field>::field_size()))
                        {
                        (left_val, right_val) => {
                            if !(*left_val == *right_val) {
                                {
                                    ::rt::begin_panic_fmt(&::std::fmt::Arguments::new_v1({
                                                                                             static __STATIC_FMTSTR:
                                                                                                    &'static [&'static str]
                                                                                                    =
                                                                                                 &["assertion failed: `(left == right)`\n  left: `",
                                                                                                   "`,\n right: `",
                                                                                                   "`: "];
                                                                                             __STATIC_FMTSTR
                                                                                         },
                                                                                         &match (&left_val,
                                                                                                 &right_val,
                                                                                                 &::std::fmt::Arguments::new_v1({
                                                                                                                                    static __STATIC_FMTSTR:
                                                                                                                                           &'static [&'static str]
                                                                                                                                           =
                                                                                                                                        &["wrong size of field"];
                                                                                                                                    __STATIC_FMTSTR
                                                                                                                                },
                                                                                                                                &match ()
                                                                                                                                     {
                                                                                                                                     ()
                                                                                                                                     =>
                                                                                                                                     [],
                                                                                                                                 }))
                                                                                              {
                                                                                              (__arg0,
                                                                                               __arg1,
                                                                                               __arg2)
                                                                                              =>
                                                                                              [::std::fmt::ArgumentV1::new(__arg0,
                                                                                                                           ::std::fmt::Debug::fmt),
                                                                                               ::std::fmt::ArgumentV1::new(__arg1,
                                                                                                                           ::std::fmt::Debug::fmt),
                                                                                               ::std::fmt::ArgumentV1::new(__arg2,
                                                                                                                           ::std::fmt::Display::fmt)],
                                                                                          }),
                                                          {
                                                              static _FILE_LINE_COL:
                                                                     (&'static str,
                                                                      u32,
                                                                      u32) =
                                                                  ("src/main.rs",
                                                                   81u32,
                                                                   9u32);
                                                              &_FILE_LINE_COL
                                                          })
                                }
                            }
                        }
                    }
                };
            };
            if true {
                {
                    match (&(64), &(64)) {
                        (left_val, right_val) => {
                            if !(*left_val == *right_val) {
                                {
                                    ::rt::begin_panic_fmt(&::std::fmt::Arguments::new_v1({
                                                                                             static __STATIC_FMTSTR:
                                                                                                    &'static [&'static str]
                                                                                                    =
                                                                                                 &["assertion failed: `(left == right)`\n  left: `",
                                                                                                   "`,\n right: `",
                                                                                                   "`: "];
                                                                                             __STATIC_FMTSTR
                                                                                         },
                                                                                         &match (&left_val,
                                                                                                 &right_val,
                                                                                                 &::std::fmt::Arguments::new_v1({
                                                                                                                                    static __STATIC_FMTSTR:
                                                                                                                                           &'static [&'static str]
                                                                                                                                           =
                                                                                                                                        &["fields should be adjacent"];
                                                                                                                                    __STATIC_FMTSTR
                                                                                                                                },
                                                                                                                                &match ()
                                                                                                                                     {
                                                                                                                                     ()
                                                                                                                                     =>
                                                                                                                                     [],
                                                                                                                                 }))
                                                                                              {
                                                                                              (__arg0,
                                                                                               __arg1,
                                                                                               __arg2)
                                                                                              =>
                                                                                              [::std::fmt::ArgumentV1::new(__arg0,
                                                                                                                           ::std::fmt::Debug::fmt),
                                                                                               ::std::fmt::ArgumentV1::new(__arg1,
                                                                                                                           ::std::fmt::Debug::fmt),
                                                                                               ::std::fmt::ArgumentV1::new(__arg2,
                                                                                                                           ::std::fmt::Display::fmt)],
                                                                                          }),
                                                          {
                                                              static _FILE_LINE_COL:
                                                                     (&'static str,
                                                                      u32,
                                                                      u32) =
                                                                  ("src/main.rs",
                                                                   81u32,
                                                                   9u32);
                                                              &_FILE_LINE_COL
                                                          })
                                }
                            }
                        }
                    }
                };
            };
            if true {
                {
                    match (&(72 - 64), &(<u64 as Field>::field_size())) {
                        (left_val, right_val) => {
                            if !(*left_val == *right_val) {
                                {
                                    ::rt::begin_panic_fmt(&::std::fmt::Arguments::new_v1({
                                                                                             static __STATIC_FMTSTR:
                                                                                                    &'static [&'static str]
                                                                                                    =
                                                                                                 &["assertion failed: `(left == right)`\n  left: `",
                                                                                                   "`,\n right: `",
                                                                                                   "`: "];
                                                                                             __STATIC_FMTSTR
                                                                                         },
                                                                                         &match (&left_val,
                                                                                                 &right_val,
                                                                                                 &::std::fmt::Arguments::new_v1({
                                                                                                                                    static __STATIC_FMTSTR:
                                                                                                                                           &'static [&'static str]
                                                                                                                                           =
                                                                                                                                        &["wrong size of field"];
                                                                                                                                    __STATIC_FMTSTR
                                                                                                                                },
                                                                                                                                &match ()
                                                                                                                                     {
                                                                                                                                     ()
                                                                                                                                     =>
                                                                                                                                     [],
                                                                                                                                 }))
                                                                                              {
                                                                                              (__arg0,
                                                                                               __arg1,
                                                                                               __arg2)
                                                                                              =>
                                                                                              [::std::fmt::ArgumentV1::new(__arg0,
                                                                                                                           ::std::fmt::Debug::fmt),
                                                                                               ::std::fmt::ArgumentV1::new(__arg1,
                                                                                                                           ::std::fmt::Debug::fmt),
                                                                                               ::std::fmt::ArgumentV1::new(__arg2,
                                                                                                                           ::std::fmt::Display::fmt)],
                                                                                          }),
                                                          {
                                                              static _FILE_LINE_COL:
                                                                     (&'static str,
                                                                      u32,
                                                                      u32) =
                                                                  ("src/main.rs",
                                                                   81u32,
                                                                   9u32);
                                                              &_FILE_LINE_COL
                                                          })
                                }
                            }
                        }
                    }
                };
            };
            if true {
                {
                    match (&(72), &(72)) {
                        (left_val, right_val) => {
                            if !(*left_val == *right_val) {
                                {
                                    ::rt::begin_panic_fmt(&::std::fmt::Arguments::new_v1({
                                                                                             static __STATIC_FMTSTR:
                                                                                                    &'static [&'static str]
                                                                                                    =
                                                                                                 &["assertion failed: `(left == right)`\n  left: `",
                                                                                                   "`,\n right: `",
                                                                                                   "`: "];
                                                                                             __STATIC_FMTSTR
                                                                                         },
                                                                                         &match (&left_val,
                                                                                                 &right_val,
                                                                                                 &::std::fmt::Arguments::new_v1({
                                                                                                                                    static __STATIC_FMTSTR:
                                                                                                                                           &'static [&'static str]
                                                                                                                                           =
                                                                                                                                        &["fields should be adjacent"];
                                                                                                                                    __STATIC_FMTSTR
                                                                                                                                },
                                                                                                                                &match ()
                                                                                                                                     {
                                                                                                                                     ()
                                                                                                                                     =>
                                                                                                                                     [],
                                                                                                                                 }))
                                                                                              {
                                                                                              (__arg0,
                                                                                               __arg1,
                                                                                               __arg2)
                                                                                              =>
                                                                                              [::std::fmt::ArgumentV1::new(__arg0,
                                                                                                                           ::std::fmt::Debug::fmt),
                                                                                               ::std::fmt::ArgumentV1::new(__arg1,
                                                                                                                           ::std::fmt::Debug::fmt),
                                                                                               ::std::fmt::ArgumentV1::new(__arg2,
                                                                                                                           ::std::fmt::Display::fmt)],
                                                                                          }),
                                                          {
                                                              static _FILE_LINE_COL:
                                                                     (&'static str,
                                                                      u32,
                                                                      u32) =
                                                                  ("src/main.rs",
                                                                   81u32,
                                                                   9u32);
                                                              &_FILE_LINE_COL
                                                          })
                                }
                            }
                        }
                    }
                };
            };
            if true {
                {
                    match (&(80), &(80)) {
                        (left_val, right_val) => {
                            if !(*left_val == *right_val) {
                                {
                                    ::rt::begin_panic_fmt(&::std::fmt::Arguments::new_v1({
                                                                                             static __STATIC_FMTSTR:
                                                                                                    &'static [&'static str]
                                                                                                    =
                                                                                                 &["assertion failed: `(left == right)`\n  left: `",
                                                                                                   "`,\n right: `",
                                                                                                   "`: "];
                                                                                             __STATIC_FMTSTR
                                                                                         },
                                                                                         &match (&left_val,
                                                                                                 &right_val,
                                                                                                 &::std::fmt::Arguments::new_v1({
                                                                                                                                    static __STATIC_FMTSTR:
                                                                                                                                           &'static [&'static str]
                                                                                                                                           =
                                                                                                                                        &["last field should matches the size of struct"];
                                                                                                                                    __STATIC_FMTSTR
                                                                                                                                },
                                                                                                                                &match ()
                                                                                                                                     {
                                                                                                                                     ()
                                                                                                                                     =>
                                                                                                                                     [],
                                                                                                                                 }))
                                                                                              {
                                                                                              (__arg0,
                                                                                               __arg1,
                                                                                               __arg2)
                                                                                              =>
                                                                                              [::std::fmt::ArgumentV1::new(__arg0,
                                                                                                                           ::std::fmt::Debug::fmt),
                                                                                               ::std::fmt::ArgumentV1::new(__arg1,
                                                                                                                           ::std::fmt::Debug::fmt),
                                                                                               ::std::fmt::ArgumentV1::new(__arg2,
                                                                                                                           ::std::fmt::Display::fmt)],
                                                                                          }),
                                                          {
                                                              static _FILE_LINE_COL:
                                                                     (&'static str,
                                                                      u32,
                                                                      u32) =
                                                                  ("src/main.rs",
                                                                   81u32,
                                                                   9u32);
                                                              &_FILE_LINE_COL
                                                          })
                                }
                            }
                        }
                    }
                };
            };
            if true {
                {
                    match (&(80 - 72), &(<u64 as Field>::field_size())) {
                        (left_val, right_val) => {
                            if !(*left_val == *right_val) {
                                {
                                    ::rt::begin_panic_fmt(&::std::fmt::Arguments::new_v1({
                                                                                             static __STATIC_FMTSTR:
                                                                                                    &'static [&'static str]
                                                                                                    =
                                                                                                 &["assertion failed: `(left == right)`\n  left: `",
                                                                                                   "`,\n right: `",
                                                                                                   "`: "];
                                                                                             __STATIC_FMTSTR
                                                                                         },
                                                                                         &match (&left_val,
                                                                                                 &right_val,
                                                                                                 &::std::fmt::Arguments::new_v1({
                                                                                                                                    static __STATIC_FMTSTR:
                                                                                                                                           &'static [&'static str]
                                                                                                                                           =
                                                                                                                                        &["wrong size of field"];
                                                                                                                                    __STATIC_FMTSTR
                                                                                                                                },
                                                                                                                                &match ()
                                                                                                                                     {
                                                                                                                                     ()
                                                                                                                                     =>
                                                                                                                                     [],
                                                                                                                                 }))
                                                                                              {
                                                                                              (__arg0,
                                                                                               __arg1,
                                                                                               __arg2)
                                                                                              =>
                                                                                              [::std::fmt::ArgumentV1::new(__arg0,
                                                                                                                           ::std::fmt::Debug::fmt),
                                                                                               ::std::fmt::ArgumentV1::new(__arg1,
                                                                                                                           ::std::fmt::Debug::fmt),
                                                                                               ::std::fmt::ArgumentV1::new(__arg2,
                                                                                                                           ::std::fmt::Display::fmt)],
                                                                                          }),
                                                          {
                                                              static _FILE_LINE_COL:
                                                                     (&'static str,
                                                                      u32,
                                                                      u32) =
                                                                  ("src/main.rs",
                                                                   81u32,
                                                                   9u32);
                                                              &_FILE_LINE_COL
                                                          })
                                }
                            }
                        }
                    }
                };
            };
        };
        let mut writer =
            MessageWriter::new(::messages::PROTOCOL_MAJOR_VERSION,
                               ::messages::TEST_NETWORK_ID, 1, 2, 80);
        writer.write(from, 0, 32);
        writer.write(to, 32, 64);
        writer.write(amount, 64, 72);
        writer.write(seed, 72, 80);
        TxTransfer{raw: RawMessage::new(writer.sign(secret_key)),}
    }
    /// Creates message and appends existing signature.
    #[allow(dead_code, unused_mut)]
    pub fn new_with_signature(from: &PublicKey, to: &PublicKey, amount: u64,
                              seed: u64, signature: &::crypto::Signature)
     -> TxTransfer {
        use ::messages::{RawMessage, MessageWriter};
        {
            use ::encoding::Field;
            if true {
                {
                    match (&(0), &(0)) {
                        (left_val, right_val) => {
                            if !(*left_val == *right_val) {
                                {
                                    ::rt::begin_panic_fmt(&::std::fmt::Arguments::new_v1({
                                                                                             static __STATIC_FMTSTR:
                                                                                                    &'static [&'static str]
                                                                                                    =
                                                                                                 &["assertion failed: `(left == right)`\n  left: `",
                                                                                                   "`,\n right: `",
                                                                                                   "`: "];
                                                                                             __STATIC_FMTSTR
                                                                                         },
                                                                                         &match (&left_val,
                                                                                                 &right_val,
                                                                                                 &::std::fmt::Arguments::new_v1({
                                                                                                                                    static __STATIC_FMTSTR:
                                                                                                                                           &'static [&'static str]
                                                                                                                                           =
                                                                                                                                        &["first field should start from 0"];
                                                                                                                                    __STATIC_FMTSTR
                                                                                                                                },
                                                                                                                                &match ()
                                                                                                                                     {
                                                                                                                                     ()
                                                                                                                                     =>
                                                                                                                                     [],
                                                                                                                                 }))
                                                                                              {
                                                                                              (__arg0,
                                                                                               __arg1,
                                                                                               __arg2)
                                                                                              =>
                                                                                              [::std::fmt::ArgumentV1::new(__arg0,
                                                                                                                           ::std::fmt::Debug::fmt),
                                                                                               ::std::fmt::ArgumentV1::new(__arg1,
                                                                                                                           ::std::fmt::Debug::fmt),
                                                                                               ::std::fmt::ArgumentV1::new(__arg2,
                                                                                                                           ::std::fmt::Display::fmt)],
                                                                                          }),
                                                          {
                                                              static _FILE_LINE_COL:
                                                                     (&'static str,
                                                                      u32,
                                                                      u32) =
                                                                  ("src/main.rs",
                                                                   81u32,
                                                                   9u32);
                                                              &_FILE_LINE_COL
                                                          })
                                }
                            }
                        }
                    }
                };
            };
            if true {
                {
                    match (&(32 - 0), &(<&PublicKey as Field>::field_size()))
                        {
                        (left_val, right_val) => {
                            if !(*left_val == *right_val) {
                                {
                                    ::rt::begin_panic_fmt(&::std::fmt::Arguments::new_v1({
                                                                                             static __STATIC_FMTSTR:
                                                                                                    &'static [&'static str]
                                                                                                    =
                                                                                                 &["assertion failed: `(left == right)`\n  left: `",
                                                                                                   "`,\n right: `",
                                                                                                   "`: "];
                                                                                             __STATIC_FMTSTR
                                                                                         },
                                                                                         &match (&left_val,
                                                                                                 &right_val,
                                                                                                 &::std::fmt::Arguments::new_v1({
                                                                                                                                    static __STATIC_FMTSTR:
                                                                                                                                           &'static [&'static str]
                                                                                                                                           =
                                                                                                                                        &["wrong size of field"];
                                                                                                                                    __STATIC_FMTSTR
                                                                                                                                },
                                                                                                                                &match ()
                                                                                                                                     {
                                                                                                                                     ()
                                                                                                                                     =>
                                                                                                                                     [],
                                                                                                                                 }))
                                                                                              {
                                                                                              (__arg0,
                                                                                               __arg1,
                                                                                               __arg2)
                                                                                              =>
                                                                                              [::std::fmt::ArgumentV1::new(__arg0,
                                                                                                                           ::std::fmt::Debug::fmt),
                                                                                               ::std::fmt::ArgumentV1::new(__arg1,
                                                                                                                           ::std::fmt::Debug::fmt),
                                                                                               ::std::fmt::ArgumentV1::new(__arg2,
                                                                                                                           ::std::fmt::Display::fmt)],
                                                                                          }),
                                                          {
                                                              static _FILE_LINE_COL:
                                                                     (&'static str,
                                                                      u32,
                                                                      u32) =
                                                                  ("src/main.rs",
                                                                   81u32,
                                                                   9u32);
                                                              &_FILE_LINE_COL
                                                          })
                                }
                            }
                        }
                    }
                };
            };
            if true {
                {
                    match (&(32), &(32)) {
                        (left_val, right_val) => {
                            if !(*left_val == *right_val) {
                                {
                                    ::rt::begin_panic_fmt(&::std::fmt::Arguments::new_v1({
                                                                                             static __STATIC_FMTSTR:
                                                                                                    &'static [&'static str]
                                                                                                    =
                                                                                                 &["assertion failed: `(left == right)`\n  left: `",
                                                                                                   "`,\n right: `",
                                                                                                   "`: "];
                                                                                             __STATIC_FMTSTR
                                                                                         },
                                                                                         &match (&left_val,
                                                                                                 &right_val,
                                                                                                 &::std::fmt::Arguments::new_v1({
                                                                                                                                    static __STATIC_FMTSTR:
                                                                                                                                           &'static [&'static str]
                                                                                                                                           =
                                                                                                                                        &["fields should be adjacent"];
                                                                                                                                    __STATIC_FMTSTR
                                                                                                                                },
                                                                                                                                &match ()
                                                                                                                                     {
                                                                                                                                     ()
                                                                                                                                     =>
                                                                                                                                     [],
                                                                                                                                 }))
                                                                                              {
                                                                                              (__arg0,
                                                                                               __arg1,
                                                                                               __arg2)
                                                                                              =>
                                                                                              [::std::fmt::ArgumentV1::new(__arg0,
                                                                                                                           ::std::fmt::Debug::fmt),
                                                                                               ::std::fmt::ArgumentV1::new(__arg1,
                                                                                                                           ::std::fmt::Debug::fmt),
                                                                                               ::std::fmt::ArgumentV1::new(__arg2,
                                                                                                                           ::std::fmt::Display::fmt)],
                                                                                          }),
                                                          {
                                                              static _FILE_LINE_COL:
                                                                     (&'static str,
                                                                      u32,
                                                                      u32) =
                                                                  ("src/main.rs",
                                                                   81u32,
                                                                   9u32);
                                                              &_FILE_LINE_COL
                                                          })
                                }
                            }
                        }
                    }
                };
            };
            if true {
                {
                    match (&(64 - 32), &(<&PublicKey as Field>::field_size()))
                        {
                        (left_val, right_val) => {
                            if !(*left_val == *right_val) {
                                {
                                    ::rt::begin_panic_fmt(&::std::fmt::Arguments::new_v1({
                                                                                             static __STATIC_FMTSTR:
                                                                                                    &'static [&'static str]
                                                                                                    =
                                                                                                 &["assertion failed: `(left == right)`\n  left: `",
                                                                                                   "`,\n right: `",
                                                                                                   "`: "];
                                                                                             __STATIC_FMTSTR
                                                                                         },
                                                                                         &match (&left_val,
                                                                                                 &right_val,
                                                                                                 &::std::fmt::Arguments::new_v1({
                                                                                                                                    static __STATIC_FMTSTR:
                                                                                                                                           &'static [&'static str]
                                                                                                                                           =
                                                                                                                                        &["wrong size of field"];
                                                                                                                                    __STATIC_FMTSTR
                                                                                                                                },
                                                                                                                                &match ()
                                                                                                                                     {
                                                                                                                                     ()
                                                                                                                                     =>
                                                                                                                                     [],
                                                                                                                                 }))
                                                                                              {
                                                                                              (__arg0,
                                                                                               __arg1,
                                                                                               __arg2)
                                                                                              =>
                                                                                              [::std::fmt::ArgumentV1::new(__arg0,
                                                                                                                           ::std::fmt::Debug::fmt),
                                                                                               ::std::fmt::ArgumentV1::new(__arg1,
                                                                                                                           ::std::fmt::Debug::fmt),
                                                                                               ::std::fmt::ArgumentV1::new(__arg2,
                                                                                                                           ::std::fmt::Display::fmt)],
                                                                                          }),
                                                          {
                                                              static _FILE_LINE_COL:
                                                                     (&'static str,
                                                                      u32,
                                                                      u32) =
                                                                  ("src/main.rs",
                                                                   81u32,
                                                                   9u32);
                                                              &_FILE_LINE_COL
                                                          })
                                }
                            }
                        }
                    }
                };
            };
            if true {
                {
                    match (&(64), &(64)) {
                        (left_val, right_val) => {
                            if !(*left_val == *right_val) {
                                {
                                    ::rt::begin_panic_fmt(&::std::fmt::Arguments::new_v1({
                                                                                             static __STATIC_FMTSTR:
                                                                                                    &'static [&'static str]
                                                                                                    =
                                                                                                 &["assertion failed: `(left == right)`\n  left: `",
                                                                                                   "`,\n right: `",
                                                                                                   "`: "];
                                                                                             __STATIC_FMTSTR
                                                                                         },
                                                                                         &match (&left_val,
                                                                                                 &right_val,
                                                                                                 &::std::fmt::Arguments::new_v1({
                                                                                                                                    static __STATIC_FMTSTR:
                                                                                                                                           &'static [&'static str]
                                                                                                                                           =
                                                                                                                                        &["fields should be adjacent"];
                                                                                                                                    __STATIC_FMTSTR
                                                                                                                                },
                                                                                                                                &match ()
                                                                                                                                     {
                                                                                                                                     ()
                                                                                                                                     =>
                                                                                                                                     [],
                                                                                                                                 }))
                                                                                              {
                                                                                              (__arg0,
                                                                                               __arg1,
                                                                                               __arg2)
                                                                                              =>
                                                                                              [::std::fmt::ArgumentV1::new(__arg0,
                                                                                                                           ::std::fmt::Debug::fmt),
                                                                                               ::std::fmt::ArgumentV1::new(__arg1,
                                                                                                                           ::std::fmt::Debug::fmt),
                                                                                               ::std::fmt::ArgumentV1::new(__arg2,
                                                                                                                           ::std::fmt::Display::fmt)],
                                                                                          }),
                                                          {
                                                              static _FILE_LINE_COL:
                                                                     (&'static str,
                                                                      u32,
                                                                      u32) =
                                                                  ("src/main.rs",
                                                                   81u32,
                                                                   9u32);
                                                              &_FILE_LINE_COL
                                                          })
                                }
                            }
                        }
                    }
                };
            };
            if true {
                {
                    match (&(72 - 64), &(<u64 as Field>::field_size())) {
                        (left_val, right_val) => {
                            if !(*left_val == *right_val) {
                                {
                                    ::rt::begin_panic_fmt(&::std::fmt::Arguments::new_v1({
                                                                                             static __STATIC_FMTSTR:
                                                                                                    &'static [&'static str]
                                                                                                    =
                                                                                                 &["assertion failed: `(left == right)`\n  left: `",
                                                                                                   "`,\n right: `",
                                                                                                   "`: "];
                                                                                             __STATIC_FMTSTR
                                                                                         },
                                                                                         &match (&left_val,
                                                                                                 &right_val,
                                                                                                 &::std::fmt::Arguments::new_v1({
                                                                                                                                    static __STATIC_FMTSTR:
                                                                                                                                           &'static [&'static str]
                                                                                                                                           =
                                                                                                                                        &["wrong size of field"];
                                                                                                                                    __STATIC_FMTSTR
                                                                                                                                },
                                                                                                                                &match ()
                                                                                                                                     {
                                                                                                                                     ()
                                                                                                                                     =>
                                                                                                                                     [],
                                                                                                                                 }))
                                                                                              {
                                                                                              (__arg0,
                                                                                               __arg1,
                                                                                               __arg2)
                                                                                              =>
                                                                                              [::std::fmt::ArgumentV1::new(__arg0,
                                                                                                                           ::std::fmt::Debug::fmt),
                                                                                               ::std::fmt::ArgumentV1::new(__arg1,
                                                                                                                           ::std::fmt::Debug::fmt),
                                                                                               ::std::fmt::ArgumentV1::new(__arg2,
                                                                                                                           ::std::fmt::Display::fmt)],
                                                                                          }),
                                                          {
                                                              static _FILE_LINE_COL:
                                                                     (&'static str,
                                                                      u32,
                                                                      u32) =
                                                                  ("src/main.rs",
                                                                   81u32,
                                                                   9u32);
                                                              &_FILE_LINE_COL
                                                          })
                                }
                            }
                        }
                    }
                };
            };
            if true {
                {
                    match (&(72), &(72)) {
                        (left_val, right_val) => {
                            if !(*left_val == *right_val) {
                                {
                                    ::rt::begin_panic_fmt(&::std::fmt::Arguments::new_v1({
                                                                                             static __STATIC_FMTSTR:
                                                                                                    &'static [&'static str]
                                                                                                    =
                                                                                                 &["assertion failed: `(left == right)`\n  left: `",
                                                                                                   "`,\n right: `",
                                                                                                   "`: "];
                                                                                             __STATIC_FMTSTR
                                                                                         },
                                                                                         &match (&left_val,
                                                                                                 &right_val,
                                                                                                 &::std::fmt::Arguments::new_v1({
                                                                                                                                    static __STATIC_FMTSTR:
                                                                                                                                           &'static [&'static str]
                                                                                                                                           =
                                                                                                                                        &["fields should be adjacent"];
                                                                                                                                    __STATIC_FMTSTR
                                                                                                                                },
                                                                                                                                &match ()
                                                                                                                                     {
                                                                                                                                     ()
                                                                                                                                     =>
                                                                                                                                     [],
                                                                                                                                 }))
                                                                                              {
                                                                                              (__arg0,
                                                                                               __arg1,
                                                                                               __arg2)
                                                                                              =>
                                                                                              [::std::fmt::ArgumentV1::new(__arg0,
                                                                                                                           ::std::fmt::Debug::fmt),
                                                                                               ::std::fmt::ArgumentV1::new(__arg1,
                                                                                                                           ::std::fmt::Debug::fmt),
                                                                                               ::std::fmt::ArgumentV1::new(__arg2,
                                                                                                                           ::std::fmt::Display::fmt)],
                                                                                          }),
                                                          {
                                                              static _FILE_LINE_COL:
                                                                     (&'static str,
                                                                      u32,
                                                                      u32) =
                                                                  ("src/main.rs",
                                                                   81u32,
                                                                   9u32);
                                                              &_FILE_LINE_COL
                                                          })
                                }
                            }
                        }
                    }
                };
            };
            if true {
                {
                    match (&(80), &(80)) {
                        (left_val, right_val) => {
                            if !(*left_val == *right_val) {
                                {
                                    ::rt::begin_panic_fmt(&::std::fmt::Arguments::new_v1({
                                                                                             static __STATIC_FMTSTR:
                                                                                                    &'static [&'static str]
                                                                                                    =
                                                                                                 &["assertion failed: `(left == right)`\n  left: `",
                                                                                                   "`,\n right: `",
                                                                                                   "`: "];
                                                                                             __STATIC_FMTSTR
                                                                                         },
                                                                                         &match (&left_val,
                                                                                                 &right_val,
                                                                                                 &::std::fmt::Arguments::new_v1({
                                                                                                                                    static __STATIC_FMTSTR:
                                                                                                                                           &'static [&'static str]
                                                                                                                                           =
                                                                                                                                        &["last field should matches the size of struct"];
                                                                                                                                    __STATIC_FMTSTR
                                                                                                                                },
                                                                                                                                &match ()
                                                                                                                                     {
                                                                                                                                     ()
                                                                                                                                     =>
                                                                                                                                     [],
                                                                                                                                 }))
                                                                                              {
                                                                                              (__arg0,
                                                                                               __arg1,
                                                                                               __arg2)
                                                                                              =>
                                                                                              [::std::fmt::ArgumentV1::new(__arg0,
                                                                                                                           ::std::fmt::Debug::fmt),
                                                                                               ::std::fmt::ArgumentV1::new(__arg1,
                                                                                                                           ::std::fmt::Debug::fmt),
                                                                                               ::std::fmt::ArgumentV1::new(__arg2,
                                                                                                                           ::std::fmt::Display::fmt)],
                                                                                          }),
                                                          {
                                                              static _FILE_LINE_COL:
                                                                     (&'static str,
                                                                      u32,
                                                                      u32) =
                                                                  ("src/main.rs",
                                                                   81u32,
                                                                   9u32);
                                                              &_FILE_LINE_COL
                                                          })
                                }
                            }
                        }
                    }
                };
            };
            if true {
                {
                    match (&(80 - 72), &(<u64 as Field>::field_size())) {
                        (left_val, right_val) => {
                            if !(*left_val == *right_val) {
                                {
                                    ::rt::begin_panic_fmt(&::std::fmt::Arguments::new_v1({
                                                                                             static __STATIC_FMTSTR:
                                                                                                    &'static [&'static str]
                                                                                                    =
                                                                                                 &["assertion failed: `(left == right)`\n  left: `",
                                                                                                   "`,\n right: `",
                                                                                                   "`: "];
                                                                                             __STATIC_FMTSTR
                                                                                         },
                                                                                         &match (&left_val,
                                                                                                 &right_val,
                                                                                                 &::std::fmt::Arguments::new_v1({
                                                                                                                                    static __STATIC_FMTSTR:
                                                                                                                                           &'static [&'static str]
                                                                                                                                           =
                                                                                                                                        &["wrong size of field"];
                                                                                                                                    __STATIC_FMTSTR
                                                                                                                                },
                                                                                                                                &match ()
                                                                                                                                     {
                                                                                                                                     ()
                                                                                                                                     =>
                                                                                                                                     [],
                                                                                                                                 }))
                                                                                              {
                                                                                              (__arg0,
                                                                                               __arg1,
                                                                                               __arg2)
                                                                                              =>
                                                                                              [::std::fmt::ArgumentV1::new(__arg0,
                                                                                                                           ::std::fmt::Debug::fmt),
                                                                                               ::std::fmt::ArgumentV1::new(__arg1,
                                                                                                                           ::std::fmt::Debug::fmt),
                                                                                               ::std::fmt::ArgumentV1::new(__arg2,
                                                                                                                           ::std::fmt::Display::fmt)],
                                                                                          }),
                                                          {
                                                              static _FILE_LINE_COL:
                                                                     (&'static str,
                                                                      u32,
                                                                      u32) =
                                                                  ("src/main.rs",
                                                                   81u32,
                                                                   9u32);
                                                              &_FILE_LINE_COL
                                                          })
                                }
                            }
                        }
                    }
                };
            };
        };
        let mut writer =
            MessageWriter::new(::messages::PROTOCOL_MAJOR_VERSION,
                               ::messages::TEST_NETWORK_ID, 1, 2, 80);
        writer.write(from, 0, 32);
        writer.write(to, 32, 64);
        writer.write(amount, 64, 72);
        writer.write(seed, 72, 80);
        TxTransfer{raw: RawMessage::new(writer.append_signature(signature)),}
    }
    #[allow(unused_variables)]
    fn check_fields(raw_message: &::messages::RawMessage)
     -> ::encoding::Result {
        let latest_segment =
            ((80 + ::messages::HEADER_LENGTH) as ::encoding::Offset).into();
        let field_from: ::encoding::Offset = 0;
        let field_to: ::encoding::Offset = 32;
        let latest_segment =
            raw_message.check::<&PublicKey>(field_from.into(),
                                            field_to.into(), latest_segment)?;
        let field_from: ::encoding::Offset = 32;
        let field_to: ::encoding::Offset = 64;
        let latest_segment =
            raw_message.check::<&PublicKey>(field_from.into(),
                                            field_to.into(), latest_segment)?;
        let field_from: ::encoding::Offset = 64;
        let field_to: ::encoding::Offset = 72;
        let latest_segment =
            raw_message.check::<u64>(field_from.into(), field_to.into(),
                                     latest_segment)?;
        let field_from: ::encoding::Offset = 72;
        let field_to: ::encoding::Offset = 80;
        let latest_segment =
            raw_message.check::<u64>(field_from.into(), field_to.into(),
                                     latest_segment)?;
        Ok(latest_segment)
    }
    /// Returns `message_id` useable for matching.
    #[allow(dead_code)]
    pub fn message_id() -> u16 { 2 }
    /// Returns `service_id` useable for matching.
    #[allow(dead_code)]
    pub fn service_id() -> u16 { 1 }
    pub fn from(&self) -> &PublicKey {
        unsafe { self.raw.read::<&PublicKey>(0, 32) }
    }
    pub fn to(&self) -> &PublicKey {
        unsafe { self.raw.read::<&PublicKey>(32, 64) }
    }
    pub fn amount(&self) -> u64 { unsafe { self.raw.read::<u64>(64, 72) } }
    pub fn seed(&self) -> u64 { unsafe { self.raw.read::<u64>(72, 80) } }
}
impl ::storage::StorageValue for TxTransfer {
    fn hash(&self) -> ::crypto::Hash { ::messages::Message::hash(self) }
    fn into_bytes(self) -> Vec<u8> { self.raw.as_ref().as_ref().to_vec() }
    fn from_bytes(value: ::std::borrow::Cow<[u8]>) -> Self {
        TxTransfer{raw:
                       ::std::sync::Arc::new(::messages::MessageBuffer::from_vec(value.into_owned())),}
    }
}
impl ::std::fmt::Debug for TxTransfer {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter)
     -> Result<(), ::std::fmt::Error> {
        fmt.debug_struct("TxTransfer").field("from",
                                             &self.from()).field("to",
                                                                 &self.to()).field("amount",
                                                                                   &self.amount()).field("seed",
                                                                                                         &self.seed()).finish()
    }
}
impl ::encoding::serialize::json::ExonumJson for TxTransfer {
    fn deserialize_field<B>(value:
                                &::encoding::serialize::json::reexport::Value,
                            buffer: &mut B, from: ::encoding::Offset,
                            to: ::encoding::Offset)
     -> Result<(), Box<::std::error::Error>> where
     B: ::encoding::serialize::WriteBufferWrapper {
        use ::encoding::serialize::json::ExonumJsonDeserialize;
        let structure = <Self as ExonumJsonDeserialize>::deserialize(value)?;
        buffer.write(from, to, structure);
        Ok(())
    }
    #[allow(unused_mut)]
    fn serialize_field(&self)
     ->
         Result<::encoding::serialize::json::reexport::Value,
                Box<::std::error::Error>> {
        use ::encoding::serialize::json::reexport::Value;
        use ::encoding::serialize::json::reexport::Map;
        let mut body = Map::new();
        body.insert("from".to_string(), self.from().serialize_field()?);
        body.insert("to".to_string(), self.to().serialize_field()?);
        body.insert("amount".to_string(), self.amount().serialize_field()?);
        body.insert("seed".to_string(), self.seed().serialize_field()?);
        let mut structure = Map::new();
        structure.insert("body".to_string(), Value::Object(body));
        structure.insert("signature".to_string(),
                         self.raw.signature().serialize_field()?);
        structure.insert("message_id".to_string(),
                         self.raw.message_type().serialize_field()?);
        structure.insert("service_id".to_string(),
                         self.raw.service_id().serialize_field()?);
        structure.insert("network_id".to_string(),
                         self.raw.network_id().serialize_field()?);
        structure.insert("protocol_version".to_string(),
                         self.raw.version().serialize_field()?);
        Ok(Value::Object(structure))
    }
}
impl ::encoding::serialize::json::ExonumJsonDeserialize for TxTransfer {
    #[allow(unused_imports, unused_variables, unused_mut)]
    fn deserialize(value: &::encoding::serialize::json::reexport::Value)
     -> Result<Self, Box<::std::error::Error>> {
        use ::encoding::serialize::json::ExonumJson;
        use ::encoding::serialize::json::reexport::from_value;
        use ::messages::{RawMessage, MessageWriter};
        let obj = value.as_object().ok_or("Can\'t cast json as object.")?;
        let body = obj.get("body").ok_or("Can\'t get body from json.")?;
        let signature =
            from_value(obj.get("signature").ok_or("Can\'t get signature from json")?.clone())?;
        let message_type =
            from_value(obj.get("message_id").ok_or("Can\'t get message_type from json")?.clone())?;
        let service_id =
            from_value(obj.get("service_id").ok_or("Can\'t get service_id from json")?.clone())?;
        let network_id =
            from_value(obj.get("network_id").ok_or("Can\'t get network_id from json")?.clone())?;
        let protocol_version =
            from_value(obj.get("protocol_version").ok_or("Can\'t get protocol_version from json")?.clone())?;
        if service_id != 1 {
            return Err("service_id didn\'t equal real service_id.".into())
        }
        if message_type != 2 {
            return Err("message_id didn\'t equal real message_id.".into())
        }
        let mut writer =
            MessageWriter::new(protocol_version, network_id, service_id,
                               message_type, 80);
        let obj = body.as_object().ok_or("Can\'t cast body as object.")?;
        let val = obj.get("from").ok_or("Can\'t get object from json.")?;
        <&PublicKey as
            ExonumJson>::deserialize_field(val, &mut writer, 0, 32)?;
        let val = obj.get("to").ok_or("Can\'t get object from json.")?;
        <&PublicKey as
            ExonumJson>::deserialize_field(val, &mut writer, 32, 64)?;
        let val = obj.get("amount").ok_or("Can\'t get object from json.")?;
        <u64 as ExonumJson>::deserialize_field(val, &mut writer, 64, 72)?;
        let val = obj.get("seed").ok_or("Can\'t get object from json.")?;
        <u64 as ExonumJson>::deserialize_field(val, &mut writer, 72, 80)?;
        Ok(TxTransfer{raw:
                          RawMessage::new(writer.append_signature(&signature)),})
    }
}
impl <'de> ::encoding::serialize::reexport::Deserialize<'de> for TxTransfer {
    #[allow(unused_mut)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where
     D: ::encoding::serialize::reexport::Deserializer<'de> {
        use ::encoding::serialize::json::reexport::Value;
        use ::encoding::serialize::reexport::{DeError, Deserialize};
        let value = <Value as Deserialize>::deserialize(deserializer)?;
        <Self as
            ::encoding::serialize::json::ExonumJsonDeserialize>::deserialize(&value).map_err(|_|
                                                                                                 D::Error::custom("Can not deserialize value."))
    }
}
impl ::encoding::serialize::reexport::Serialize for TxTransfer {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where
     S: ::encoding::serialize::reexport::Serializer {
        use ::encoding::serialize::reexport::SerError;
        use ::encoding::serialize::json::ExonumJson;
        self.serialize_field().map_err(|_|
                                           S::Error::custom("Can not serialize structure: TxTransfer"))?.serialize(serializer)
    }
}
impl Transaction for TxTransfer {
    fn verify(&self) -> bool {
        self.verify_signature(self.from()) && TransactionVerify::verify(self)
    }
    fn execute(&self, view: &mut Fork) {
        TransactionExecute::execute(self, view);
    }
}
impl TransactionVerify for TxTransfer {
    fn verify(&self) -> bool {
        (*self.from() != *self.to()) && (self.amount() > 0)
    }
}
impl TransactionExecute for TxTransfer {
    fn execute(&self, view: &mut Fork) {
        let mut schema = WalletSchema{view,};
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
    fn verify(&self) -> bool { true }
}
impl TransactionExecute for TxCreateWallet {
    fn execute(&self, view: &mut Fork) {
        let mut schema = WalletSchema{view,};
        if schema.get(self.pub_key()).is_none() {
            let wallet = Wallet::new(self.pub_key(), self.name(), 1000u64);
            ::io::_print(::std::fmt::Arguments::new_v1({
                                                           static __STATIC_FMTSTR:
                                                                  &'static [&'static str]
                                                                  =
                                                               &["Create the wallet: ",
                                                                 "\n"];
                                                           __STATIC_FMTSTR
                                                       },
                                                       &match (&wallet,) {
                                                            (__arg0,) =>
                                                            [::std::fmt::ArgumentV1::new(__arg0,
                                                                                         ::std::fmt::Debug::fmt)],
                                                        }));
            schema.index().put(self.pub_key(), wallet)
        }
    }
}
fn main() {
    exonum::helpers::init_logger().unwrap();
    ::io::_print(::std::fmt::Arguments::new_v1({
                                                   static __STATIC_FMTSTR:
                                                          &'static [&'static str]
                                                          =
                                                       &["Creating in-memory database...\n"];
                                                   __STATIC_FMTSTR
                                               }, &match () { () => [], }));
    let db = MemoryDB::new();
    let services: Vec<Box<Service>> =
        <[_]>::into_vec(box [Box::new(WalletService)]);
    let blockchain = Blockchain::new(Box::new(db), services);
    let (consensus_public_key, consensus_secret_key) =
        exonum::crypto::gen_keypair();
    let (service_public_key, service_secret_key) =
        exonum::crypto::gen_keypair();
    let peer_address = "0.0.0.0:2000".parse().unwrap();
    let api_address = "0.0.0.0:8000".parse().unwrap();
    let validator_keys =
        ValidatorKeys{consensus_key: consensus_public_key,
                      service_key: service_public_key,};
    let genesis =
        GenesisConfig::new(<[_]>::into_vec(box [validator_keys]).into_iter());
    let api_cfg =
        NodeApiConfig{public_api_address:
                          Some(api_address), ..Default::default()};
    let node_cfg =
        NodeConfig{listen_address: peer_address,
                   peers: <[_]>::into_vec(box []),
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
                   services_configs: Default::default(),};
    ::io::_print(::std::fmt::Arguments::new_v1({
                                                   static __STATIC_FMTSTR:
                                                          &'static [&'static str]
                                                          =
                                                       &["Starting a single node...\n"];
                                                   __STATIC_FMTSTR
                                               }, &match () { () => [], }));
    let mut node = Node::new(blockchain, node_cfg);
    ::io::_print(::std::fmt::Arguments::new_v1({
                                                   static __STATIC_FMTSTR:
                                                          &'static [&'static str]
                                                          =
                                                       &["Blockchain in ready for transactions!\n"];
                                                   __STATIC_FMTSTR
                                               }, &match () { () => [], }));
    node.run().unwrap();
}
