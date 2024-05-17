pub mod guards;
pub mod init;
pub mod manage;
pub mod memory;
pub mod query;
pub mod types;
pub mod update;
pub mod utils;

use icrc_ledger_types::icrc1::account::Account;
use memory::Settings;
use serde_bytes::ByteBuf;
use std::collections::BTreeSet;
use std::collections::HashMap;
use types::icrc37::*;
use types::icrc7::*;
use types::*;

// pub static ANONYMOUS: Principal = Principal::anonymous();

// pub fn is_controller() -> Result<()> {
//     if ic_cdk::api::is_controller(&ic_cdk::caller()) {
//         Ok(())
//     } else {
//         Err("user is not a controller".to_string())
//     }
// }

// pub fn is_authenticated() -> Result<()> {
//     if ic_cdk::caller() == ANONYMOUS {
//         Err("anonymous user is not allowed".to_string())
//     } else {
//         Ok(())
//     }
// }

// ic_cdk::export_candid!();

#[derive(thiserror::Error, Debug)]
pub enum Error {}

pub type Result<T> = std::result::Result<T, Error>;

pub trait Icrc7 {
    fn collection_metadata(&self) -> Result<&HashMap<String, Value>>;
    fn symbol(&self) -> Result<&str>;
    fn name(&self) -> Result<&str>;
    fn description(&self) -> Result<Option<&str>>;
    fn logo(&self) -> Result<Option<&str>>;
    fn total_supply(&self) -> Result<usize>;
    fn supply_cap(&self) -> Result<Option<usize>>;
    fn max_query_batch_size(&self) -> Result<Option<usize>>;
    fn max_update_batch_size(&self) -> Result<Option<usize>>;
    fn default_take_value(&self) -> Result<Option<usize>>;
    fn max_take_value(&self) -> Result<Option<usize>>;
    fn max_memo_size(&self) -> Result<Option<usize>>;
    fn atomic_batch_transfers(&self) -> Result<Option<bool>>;
    fn tx_window(&self) -> Result<Option<usize>>;
    fn permitted_drift(&self) -> Result<Option<usize>>;
    fn token_metadata(&self, token_ids: Vec<usize>)
        -> Result<Vec<Option<&HashMap<String, Value>>>>;
    fn owner_of(&self, token_ids: Vec<usize>) -> Result<Vec<Option<Account>>>;
    fn balance_of(&self, accounts: Vec<Account>) -> Result<Vec<usize>>;
    fn tokens(&self, prev: Option<usize>, take: Option<usize>) -> Result<Vec<usize>>;
    fn tokens_of(
        &self,
        account: Account,
        prev: Option<usize>,
        take: Option<usize>,
    ) -> Result<Vec<usize>>;
    fn transfer(&mut self, args: Vec<TransferArg>) -> Result<Vec<Option<TransferResult>>>;
    fn mint(&mut self, args: MintArg) -> Result<MintResult>;
}

pub trait Icrc37 {
    fn metadata(&self) -> Result<Metadata>;
    fn max_approvals_per_token_or_collection(&self) -> Result<Option<usize>>;
    fn max_revoke_approvals(&self) -> Result<Option<usize>>;
    fn is_approved(&self, args: Vec<IsApprovedArg>) -> Result<Vec<bool>>;
    fn get_token_approvals(
        &self,
        token_id: usize,
        prev: Option<TokenApproval>,
        take: Option<usize>,
    ) -> Result<Vec<TokenApproval>>;
    fn get_collection_approvals(
        &self,
        owner: Account,
        prev: Option<CollectionApproval>,
        take: Option<usize>,
    ) -> Result<Vec<CollectionApproval>>;
    fn approve_tokens(
        &mut self,
        args: Vec<ApproveTokenArg>,
    ) -> Result<Vec<Option<ApproveTokenResult>>>;
    fn approve_collection(
        &mut self,
        args: Vec<ApproveCollectionArg>,
    ) -> Result<Vec<Option<ApproveCollectionResult>>>;
    fn revoke_token_approvals(
        &mut self,
        args: Vec<RevokeTokenApprovalArg>,
    ) -> Result<Vec<Option<RevokeTokenApprovalResult>>>;
    fn revoke_collection_approvals(
        &mut self,
        args: Vec<RevokeCollectionApprovalArg>,
    ) -> Result<Vec<Option<RevokeCollectionApprovalResult>>>;
    fn transfer_from(
        &mut self,
        args: Vec<TransferFromArg>,
    ) -> Result<Vec<Option<TransferFromResult>>>;
}

pub trait Icrc10 {
    fn supported_standards(&self) -> Result<Vec<Standard>>;
}

pub trait Collection: Icrc7 + Icrc37 + Icrc10 {
    fn set_minters(args: BTreeSet<Account>) -> Result<()>;
    fn set_managers(args: BTreeSet<Account>) -> Result<()>;
    fn update_collection(args: UpdateCollectionArg) -> Result<()>;
    fn secret(args: SecretArg) -> Result<ByteBuf>;
    fn create_token(args: CreateTokenArg) -> Result<usize>;
    fn create_token_by_secret(args: CreateTokenArg) -> Result<usize>;
    fn update_token(args: UpdateTokenArg) -> Result<()>;
}

pub struct Coll {
    pub symbol: String,
    pub name: String,
    pub description: Option<String>,
    pub logo: Option<String>,
    pub assets_origin: Option<String>,
    pub total_supply: u64,
    pub supply_cap: Option<u64>,
    pub created_at: u64,
    pub updated_at: u64,
    pub minters: BTreeSet<Account>,
    pub managers: BTreeSet<Account>,
    pub settings: Settings,
}
