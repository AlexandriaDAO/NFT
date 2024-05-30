use std::collections::BTreeSet;

use candid::{CandidType, Nat, Principal};
use icrc_ledger_types::icrc1::{
    account::{Account, Subaccount},
    transfer::Memo,
};
use serde::{Deserialize, Serialize};
use serde_bytes::ByteBuf;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("{0}")]
    Custom(&'static str),
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct TransferArg {
    pub from_subaccount: Option<Subaccount>,
    pub to: Account,
    pub token_id: Nat,
    pub memo: Option<Memo>,
    pub created_at_time: Option<u64>,
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub enum TransferError {
    NonExistingTokenId,
    InvalidRecipient,
    Unauthorized,
    TooOld,
    CreatedInFuture { ledger_time: u64 },
    Duplicate { duplicate_of: Nat },
    GenericError { error_code: Nat, message: String },
    GenericBatchError { error_code: Nat, message: String },
}

pub type TransferResult = std::result::Result<Nat, TransferError>;

#[derive(CandidType, Deserialize)]
pub struct CreateTokenArg {
    pub name: String,
    pub description: Option<String>,
    pub asset_name: String,
    pub asset_content_type: String,
    pub asset_content: ByteBuf,
    pub metadata: String,
    pub supply_cap: Option<u32>,
    pub author: Principal,
    pub challenge: Option<ByteBuf>,
}

#[derive(CandidType, Deserialize)]
pub struct UpdateTokenArg {
    pub id: Nat,
    pub name: Option<String>,
    pub description: Option<String>,
    pub asset_name: Option<String>,
    pub asset_content_type: Option<String>,
    pub asset_content: Option<ByteBuf>,
    pub metadata: Option<String>,
    pub supply_cap: Option<u32>,
    pub author: Option<Principal>,
}

#[derive(CandidType, Serialize)]
pub struct Standard {
    pub name: String,
    pub url: String,
}

#[derive(CandidType, Deserialize, Clone)]
pub struct MintArg {
    pub token_id: Nat,
    pub holders: BTreeSet<Principal>,
}

#[derive(CandidType, Serialize, Clone)]
pub enum MintError {
    NonExistingTokenId,
    SupplyCapReached,
    GenericBatchError { error_code: Nat, message: String },
}

pub type MintResult = std::result::Result<Nat, MintError>;

pub trait Icrc7Trait {
    fn icrc7_collection_metadata(&self) -> serde_json::Value;
    fn symbol(&self) -> Result<&str>;
    fn name(&self) -> Result<&str>;
    fn description(&self) -> Result<&str>;
    fn logo(&self) -> Result<&str>;
    fn total_supply(&self) -> Result<usize>;
    fn supply_cap(&self) -> Result<Option<usize>> {
        Ok(None)
    }
    fn max_query_batch_size(&self) -> Result<usize> {
        Ok(100)
    }
    fn max_update_batch_size(&self) -> Result<usize> {
        Ok(20)
    }
    fn default_take_value(&self) -> Result<usize> {
        Ok(10)
    }
    fn max_take_value(&self) -> Result<usize> {
        Ok(100)
    }
    fn max_memo_size(&self) -> Result<usize> {
        Ok(32)
    }
    fn atomic_batch_transfers(&self) -> Result<bool> {
        Ok(false)
    }
    fn tx_window(&self) -> Result<usize> {
        Ok(2 * 60 * 60)
    }
    fn permitted_drift(&self) -> Result<usize> {
        Ok(2 * 60)
    }
    fn token_metadata(&self, token_ids: Vec<i64>) -> Result<Vec<serde_json::Value>>;
    fn owner_of(&self, token_ids: Vec<i64>) -> Result<Vec<Option<Principal>>>;
    fn balance_of(&self, accounts: Vec<Principal>) -> Result<Vec<usize>>;
    fn tokens(&self, prev: Option<usize>, take: Option<usize>) -> Result<Vec<i64>>;
    fn tokens_of(
        &self,
        account: Principal,
        prev: Option<usize>,
        take: Option<usize>,
    ) -> Result<Vec<i64>>;

    // fn create_token(&mut self, args: CreateTokenArg) -> Result<Nat>;
    // fn update_token(args: UpdateTokenArg) -> Result<()>;
    // fn mint(&mut self, args: MintArg) -> Result<MintResult>;
    // fn transfer(&mut self, args: Vec<TransferArg>) -> Result<Vec<Option<TransferResult>>>;
}

pub trait Icrc10Trait {
    fn supported_standards(&self) -> Result<Vec<Standard>>;
}

// pub struct Coll {
//     pub symbol: String,
//     pub name: String,
//     pub description: Option<String>,
//     pub logo: Option<String>,
//     pub assets_origin: Option<String>,
//     pub total_supply: u64,
//     pub supply_cap: Option<u64>,
//     pub created_at: u64,
//     pub updated_at: u64,
//     pub minters: BTreeSet<Account>,
//     pub managers: BTreeSet<Account>,
//     pub settings: Settings,
// }

pub use icp_token_derive::Icrc7;
