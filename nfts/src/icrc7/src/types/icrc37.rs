use candid::{CandidType, Nat};
use icrc_ledger_types::icrc1::{
    account::{Account, Subaccount},
    transfer::Memo,
};
use serde::{Deserialize, Serialize};
use std::string::ToString;

use super::Metadata;

#[derive(CandidType, Deserialize, Serialize)]
pub struct ApprovalInfo {
    pub spender: Account,
    pub from_subaccount: Option<Subaccount>,
    pub expires_at: Option<u64>,
    pub created_at_time: Option<u64>,
    pub memo: Option<Memo>,
}

#[derive(CandidType, Deserialize)]
pub struct ApproveTokenArg {
    pub token_id: Nat,
    pub approval_info: ApprovalInfo,
}

pub type ApproveTokenResult = Result<Nat, ApproveTokenError>;

#[derive(CandidType, Serialize, Clone)]
pub enum ApproveTokenError {
    InvalidSpender,
    Unauthorized,
    NonExistingTokenId,
    TooOld,
    CreatedInFuture { ledger_time: u64 },
    GenericError { error_code: Nat, message: String },
    GenericBatchError { error_code: Nat, message: String },
}

#[derive(CandidType, Deserialize)]
pub struct ApproveCollectionArg {
    pub approval_info: ApprovalInfo,
}

pub type ApproveCollectionResult = Result<Nat, ApproveCollectionError>;

#[derive(CandidType, Serialize, Clone)]
pub enum ApproveCollectionError {
    InvalidSpender,
    TooOld,
    CreatedInFuture { ledger_time: u64 },
    GenericError { error_code: Nat, message: String },
    GenericBatchError { error_code: Nat, message: String },
}

#[derive(CandidType, Deserialize)]
pub struct RevokeTokenApprovalArg {
    pub spender: Option<Account>,
    pub from_subaccount: Option<Subaccount>,
    pub token_id: Nat,
    pub memo: Option<Memo>,
    pub created_at_time: Option<u64>,
}

pub type RevokeTokenApprovalResult = Result<Nat, RevokeTokenApprovalError>;

#[derive(CandidType, Serialize, Clone)]
pub enum RevokeTokenApprovalError {
    ApprovalDoesNotExist,
    Unauthorized,
    NonExistingTokenId,
    TooOld,
    CreatedInFuture { ledger_time: u64 },
    GenericError { error_code: Nat, message: String },
    GenericBatchError { error_code: Nat, message: String },
}

#[derive(CandidType, Deserialize)]
pub struct RevokeCollectionApprovalArg {
    pub spender: Option<Account>,
    pub from_subaccount: Option<Subaccount>,
    pub memo: Option<Memo>,
    pub created_at_time: Option<u64>,
}

pub type RevokeCollectionApprovalResult = Result<Nat, RevokeCollectionApprovalError>;

#[derive(CandidType, Serialize, Clone)]
pub enum RevokeCollectionApprovalError {
    ApprovalDoesNotExist,
    TooOld,
    CreatedInFuture { ledger_time: u64 },
    GenericError { error_code: Nat, message: String },
    GenericBatchError { error_code: Nat, message: String },
}

#[derive(CandidType, Deserialize)]
pub struct IsApprovedArg {
    pub spender: Account,
    pub from_subaccount: Option<Subaccount>,
    pub token_id: Nat,
}

#[derive(CandidType, Deserialize, Serialize)]
pub struct TokenApproval {
    pub token_id: Nat,
    pub approval_info: ApprovalInfo,
}

pub type CollectionApproval = ApprovalInfo;

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct TransferFromArg {
    pub spender_subaccount: Option<Subaccount>,
    pub from: Account,
    pub to: Account,
    pub token_id: Nat,
    pub memo: Option<Memo>,
    pub created_at_time: Option<u64>,
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub enum TransferFromError {
    NonExistingTokenId,
    InvalidRecipient,
    Unauthorized,
    TooOld,
    CreatedInFuture { ledger_time: u64 },
    Duplicate { duplicate_of: Nat },
    GenericError { error_code: Nat, message: String },
    GenericBatchError { error_code: Nat, message: String },
}

pub type TransferFromResult = Result<Nat, TransferFromError>;

#[derive(CandidType, Serialize, Clone)]
pub struct Transaction {
    pub ts: Nat,
    pub op: String,
    pub tid: Nat,
    pub from: Option<Account>,
    pub to: Option<Account>,
    pub spender: Option<Account>,
    pub exp: Option<Nat>,
    pub meta: Option<Metadata>,
    pub memo: Option<Memo>,
}
