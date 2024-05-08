use candid::Principal;
use ciborium::{from_reader, into_writer};
use ic_stable_structures::{storable::Bound, StableBTreeMap, Storable};
use icrc_ledger_types::icrc1::account::Account;
use serde::{Deserialize, Serialize};
use std::{borrow::Cow, collections::BTreeMap};

use crate::types::{
    icrc37::{ApprovalInfo, RevokeCollectionApprovalError, RevokeCollectionApprovalResult},
    TokenId,
};

use super::{Memory, OWNER_APPROVALS};

#[derive(Clone, Default, Deserialize, Serialize)]
pub struct Approvals(pub BTreeMap<Principal, (u64, u64)>);
pub type ApprovalItem<'a> = (&'a Principal, &'a (u64, u64));

impl Storable for Approvals {
    const BOUND: Bound = Bound::Unbounded;

    fn to_bytes(&self) -> Cow<[u8]> {
        let mut buf = vec![];
        into_writer(&self.0, &mut buf).unwrap();
        Cow::Owned(buf)
    }

    fn from_bytes(bytes: Cow<'_, [u8]>) -> Self {
        from_reader(&bytes[..]).unwrap()
    }
}

impl Approvals {
    pub fn to_info(item: ApprovalItem) -> ApprovalInfo {
        ApprovalInfo {
            spender: Account {
                owner: *item.0,
                subaccount: None,
            },
            from_subaccount: None,
            created_at_time: if item.1 .0 > 0 { Some(item.1 .0) } else { None },
            expires_at: if item.1 .1 > 0 { Some(item.1 .1) } else { None },
            memo: None,
        }
    }

    pub fn total(&self) -> u32 {
        self.0.len() as u32
    }

    pub fn iter(&self) -> impl Iterator<Item = ApprovalItem> {
        self.0.iter()
    }

    pub fn get(&self, spender: &Principal) -> Option<(u64, u64)> {
        self.0.get(spender).cloned()
    }

    pub fn insert(&mut self, spender: Principal, create_at_sec: u64, exp_sec: u64) {
        self.0.insert(spender, (create_at_sec, exp_sec));
    }

    pub fn revoke(&mut self, spender: &Principal) -> Option<(u64, u64)> {
        self.0.remove(spender)
    }
}

pub fn is_approved(from: &Principal, spender: &Principal, now_sec: u64) -> bool {
    with(|r| {
        if let Some(approvals) = r.get(from) {
            if let Some((_, expire_at)) = approvals.0.get(spender) {
                return expire_at > &now_sec;
            }
        }
        false
    })
}

// used by atomic_batch_transfers checking
pub fn find_unapproved<'a>(
    spender: &Principal,
    args: &'a [(TokenId, &Principal)],
    now_sec: u64,
) -> Vec<&'a (TokenId, &'a Principal)> {
    with(|r| {
        args.iter()
            .filter(|(_, from)| match r.get(from) {
                None => true,
                Some(approvals) => match approvals.0.get(spender) {
                    None => true,
                    Some((_, expire_at)) => expire_at <= &now_sec,
                },
            })
            .collect()
    })
}

pub fn spenders_is_approved(from: &Principal, spenders: &[&Principal], now_sec: u64) -> Vec<bool> {
    with(|r| {
        let mut res = vec![false; spenders.len()];
        if let Some(approvals) = r.get(from) {
            for (i, spender) in spenders.iter().enumerate() {
                if let Some((_, expire_at)) = approvals.0.get(spender) {
                    res[i] = expire_at > &now_sec;
                }
            }
        }
        res
    })
}

pub fn revoke(
    from: &Principal,
    spenders: &[Option<Principal>],
) -> Vec<Option<RevokeCollectionApprovalResult>> {
    with_mut(|r| {
        let mut res: Vec<Option<RevokeCollectionApprovalResult>> = vec![None; spenders.len()];
        if let Some(mut approvals) = r.get(from) {
            for (i, spender) in spenders.iter().enumerate() {
                match spender {
                    Some(spender) => {
                        if approvals.0.remove(spender).is_none() {
                            res[i] = Some(Err(RevokeCollectionApprovalError::ApprovalDoesNotExist));
                        }
                    }
                    None => {
                        r.remove(from);
                        return res; // no need to continue
                    }
                }
            }
        } else {
            res.fill(Some(Err(
                RevokeCollectionApprovalError::ApprovalDoesNotExist,
            )));
        };

        res
    })
}

pub fn with<R>(f: impl FnOnce(&StableBTreeMap<Principal, Approvals, Memory>) -> R) -> R {
    OWNER_APPROVALS.with(|r| f(&r.borrow()))
}

pub fn with_mut<R>(f: impl FnOnce(&mut StableBTreeMap<Principal, Approvals, Memory>) -> R) -> R {
    OWNER_APPROVALS.with(|r| f(&mut r.borrow_mut()))
}
