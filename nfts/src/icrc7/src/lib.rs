pub mod guards;
pub mod init;
pub mod manage;
pub mod memory;
pub mod query;
pub mod types;
pub mod update;
pub mod utils;

use candid::{Nat, Principal};
use icrc_ledger_types::icrc1::account::Account;
use serde_bytes::ByteBuf;
use std::collections::BTreeSet;
use types::icrc37::*;
use types::icrc7::*;
use types::*;

pub static ANONYMOUS: Principal = Principal::anonymous();

pub fn is_controller() -> Result<(), String> {
    if ic_cdk::api::is_controller(&ic_cdk::caller()) {
        Ok(())
    } else {
        Err("user is not a controller".to_string())
    }
}

pub fn is_authenticated() -> Result<(), String> {
    if ic_cdk::caller() == ANONYMOUS {
        Err("anonymous user is not allowed".to_string())
    } else {
        Ok(())
    }
}

ic_cdk::export_candid!();
