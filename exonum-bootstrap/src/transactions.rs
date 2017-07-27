use exonum::storage::{Fork};

pub trait TransactionVerify { fn verify(&self) -> bool; }
pub trait TransactionExecute { fn execute(&self, view: &mut Fork); }

pub trait CheckIfTransactionVerifyImpl : TransactionVerify { }
pub trait CheckIfTransactionExecuteImpl : TransactionExecute { }
