use std::borrow::Cow;

use candid::Principal;
use ciborium::{from_reader, into_writer};
use ic_stable_structures::{storable::Bound, Storable};
use icrc_ledger_types::icrc1::transfer::Memo;
use serde::{Deserialize, Serialize};

use crate::types::Metadata;

use super::TRANSACTIONS;

#[derive(Clone, Default, Deserialize, Serialize)]
pub struct Transaction {
    pub ts: u64,
    pub op: String,
    pub tid: u64,
    pub from: Option<Principal>,
    pub to: Option<Principal>,
    pub spender: Option<Principal>,
    pub exp: Option<u64>,
    pub meta: Option<Metadata>,
    pub memo: Option<Memo>,
}

impl Transaction {
    pub fn mint(
        now_sec: u64,
        tid: u64,
        from: Option<Principal>,
        to: Principal,
        meta: Metadata,
        memo: Option<Memo>,
    ) -> Self {
        Transaction {
            ts: now_sec,
            op: "7mint".to_string(),
            tid,
            from,
            to: Some(to),
            meta: Some(meta),
            memo,
            ..Default::default()
        }
    }

    pub fn burn(
        now_sec: u64,
        tid: u64,
        from: Principal,
        to: Option<Principal>,
        memo: Option<Memo>,
    ) -> Self {
        Transaction {
            ts: now_sec,
            op: "7burn".to_string(),
            tid,
            from: Some(from),
            to,
            memo,
            ..Default::default()
        }
    }

    pub fn transfer(
        now_sec: u64,
        tid: u64,
        from: Principal,
        to: Principal,
        memo: Option<Memo>,
    ) -> Self {
        Transaction {
            ts: now_sec,
            op: "7xfer".to_string(),
            tid,
            from: Some(from),
            to: Some(to),
            memo,
            ..Default::default()
        }
    }

    pub fn update(
        now_sec: u64,
        tid: u64,
        from: Principal,
        meta: Metadata,
        memo: Option<Memo>,
    ) -> Self {
        Transaction {
            ts: now_sec,
            op: "7update".to_string(),
            tid,
            from: Some(from),
            meta: Some(meta),
            memo,
            ..Default::default()
        }
    }

    pub fn approve(
        now_sec: u64,
        tid: u64,
        from: Principal,
        spender: Principal,
        exp_sec: Option<u64>,
        memo: Option<Memo>,
    ) -> Self {
        Transaction {
            ts: now_sec,
            op: "37appr".to_string(),
            tid,
            from: Some(from),
            spender: Some(spender),
            exp: exp_sec,
            memo,
            ..Default::default()
        }
    }

    pub fn approve_collection(
        now_sec: u64,
        from: Principal,
        spender: Principal,
        exp_sec: Option<u64>,
        memo: Option<Memo>,
    ) -> Self {
        Transaction {
            ts: now_sec,
            op: "37appr_coll".to_string(),
            from: Some(from),
            spender: Some(spender),
            exp: exp_sec,
            memo,
            ..Default::default()
        }
    }

    pub fn revoke(
        now_sec: u64,
        tid: u64,
        from: Principal,
        spender: Option<Principal>,
        memo: Option<Memo>,
    ) -> Self {
        Transaction {
            ts: now_sec,
            op: "37revoke".to_string(),
            tid,
            from: Some(from),
            spender,
            memo,
            ..Default::default()
        }
    }

    pub fn revoke_collection(
        now_sec: u64,
        from: Principal,
        spender: Option<Principal>,
        memo: Option<Memo>,
    ) -> Self {
        Transaction {
            ts: now_sec,
            op: "37revoke_coll".to_string(),
            from: Some(from),
            spender,
            memo,
            ..Default::default()
        }
    }

    pub fn transfer_from(
        now_sec: u64,
        tid: u64,
        from: Principal,
        to: Principal,
        spender: Principal,
        memo: Option<Memo>,
    ) -> Self {
        Transaction {
            ts: now_sec,
            op: "37xfer".to_string(),
            tid,
            from: Some(from),
            to: Some(to),
            spender: Some(spender),
            memo,
            ..Default::default()
        }
    }
}

impl Storable for Transaction {
    const BOUND: Bound = Bound::Unbounded;

    fn to_bytes(&self) -> Cow<[u8]> {
        let mut buf = vec![];
        into_writer(self, &mut buf).unwrap();
        Cow::Owned(buf)
    }

    fn from_bytes(bytes: Cow<'_, [u8]>) -> Self {
        from_reader(&bytes[..]).unwrap()
    }
}

pub fn total() -> u64 {
    TRANSACTIONS.with(|r| r.borrow().len())
}

pub fn append(tx: &Transaction) -> Result<u64, String> {
    TRANSACTIONS
        .with(|r| r.borrow_mut().append(tx))
        .map_err(|err| format!("failed to append transaction log, error {:?}", err))
}
