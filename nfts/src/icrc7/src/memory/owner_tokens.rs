use candid::{Nat, Principal};
use ciborium::{from_reader, into_writer};
use ic_stable_structures::{storable::Bound, StableBTreeMap, Storable};
use serde::{Deserialize, Serialize};
use std::{borrow::Cow, collections::BTreeMap};

use crate::types::{
    icrc37::{ApproveTokenError, RevokeTokenApprovalError},
    TokenId,
};

use super::{approvals::Approvals, Memory, OWNER_TOKENS};

#[derive(Clone, Default, Deserialize, Serialize)]
pub struct OwnerTokens(BTreeMap<u32, BTreeMap<u32, Option<Approvals>>>);

impl Storable for OwnerTokens {
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

impl OwnerTokens {
    pub fn balance_of(&self) -> u64 {
        self.0.values().map(|records| records.len() as u64).sum()
    }

    pub fn token_ids(&self) -> Vec<u32> {
        self.0.keys().cloned().collect()
    }

    pub fn get_sids(&self, tid: u32) -> Option<Vec<u32>> {
        self.0
            .get(&tid)
            .map(|records| records.keys().cloned().collect())
    }

    pub fn clear_for_transfer(&mut self, tid: u32, sid: u32) -> usize {
        if let Some(records) = self.0.get_mut(&tid) {
            records.remove(&sid);
            if records.is_empty() {
                self.0.remove(&tid);
            }
        }
        self.0.len()
    }

    pub fn get_approvals(&self, tid: u32, sid: u32) -> Option<&Approvals> {
        if let Some(records) = self.0.get(&tid) {
            records.get(&sid).and_then(|approvals| approvals.as_ref())
        } else {
            None
        }
    }

    pub fn insert_approvals(
        &mut self,
        max_approvals: u16,
        tid: u32,
        sid: u32,
        spender: Principal,
        create_at_sec: u64,
        exp_sec: u64,
    ) -> Result<(), ApproveTokenError> {
        match self.0.get_mut(&tid) {
            None => Err(ApproveTokenError::NonExistingTokenId),
            Some(records) => match records.get_mut(&sid) {
                None => Err(ApproveTokenError::NonExistingTokenId),
                Some(None) => {
                    let mut approvals = Approvals::default();
                    approvals.insert(spender, create_at_sec, exp_sec);
                    records.insert(sid, Some(approvals));
                    Ok(())
                }
                Some(Some(approvals)) => {
                    if approvals.total() >= max_approvals as u32 {
                        Err(ApproveTokenError::GenericBatchError {
                            error_code: Nat::from(0u64),
                            message: "exceeds the maximum number of approvals".to_string(),
                        })
                    } else {
                        approvals.insert(spender, create_at_sec, exp_sec);
                        Ok(())
                    }
                }
            },
        }
    }

    pub fn revoke(
        &mut self,
        tid: u32,
        sid: u32,
        spender: Option<Principal>,
    ) -> Result<(), RevokeTokenApprovalError> {
        if let Some(records) = self.0.get_mut(&tid) {
            if let Some(approvals) = records.get_mut(&sid) {
                match spender {
                    Some(spender) => match approvals {
                        Some(approvals) => {
                            if approvals.0.remove(&spender).is_none() {
                                return Err(RevokeTokenApprovalError::ApprovalDoesNotExist);
                            }
                            return Ok(());
                        }
                        None => {
                            return Err(RevokeTokenApprovalError::ApprovalDoesNotExist);
                        }
                    },
                    None => {
                        *approvals = None;
                    }
                }
            }
        }

        Err(RevokeTokenApprovalError::NonExistingTokenId)
    }
}

pub fn is_approved(
    from: &Principal,
    spender: &Principal,
    tid: u32,
    sid: u32,
    now_sec: u64,
) -> bool {
    with(|r| {
        if let Some(tokens) = r.get(from) {
            if let Some(records) = tokens.0.get(&tid) {
                if let Some(Some(approvals)) = records.get(&sid) {
                    return approvals
                        .0
                        .get(spender)
                        .map_or(false, |(_, expire_at)| expire_at > &now_sec);
                }
            }
        }
        false
    })
}

pub fn spenders_is_approved(
    from: &Principal,
    args: &[(TokenId, &Principal)],
    now_sec: u64,
) -> Vec<bool> {
    with(|r| {
        let mut res = vec![false; args.len()];
        if let Some(tokens) = r.get(from) {
            for (i, (id, spender)) in args.iter().enumerate() {
                if let Some(records) = tokens.0.get(&id.0) {
                    if let Some(Some(approvals)) = records.get(&id.1) {
                        res[i] = approvals
                            .0
                            .get(spender)
                            .map_or(false, |(_, expire_at)| expire_at > &now_sec);
                    }
                }
            }
        }
        res
    })
}

pub fn all_is_approved<'a>(
    spender: &Principal,
    args: &'a [&(TokenId, &Principal)],
    now_sec: u64,
) -> Result<(), &'a Principal> {
    with(|r| {
        for arg in args.iter() {
            match r.get(arg.1) {
                None => return Err(arg.1),
                Some(tokens) => match tokens.0.get(&arg.0 .0) {
                    None => return Err(arg.1),
                    Some(records) => match records.get(&arg.0 .1) {
                        None => return Err(arg.1),
                        Some(None) => return Err(arg.1),
                        Some(Some(approvals)) => match approvals.get(spender) {
                            None => return Err(arg.1),
                            Some((_, expire_at)) => {
                                if expire_at <= now_sec {
                                    return Err(arg.1);
                                }
                            }
                        },
                    },
                },
            }
        }

        Ok(())
    })
}

pub fn update_for_transfer(from: Principal, to: Principal, tid: u32, sid: u32) {
    with_mut(|r| {
        if let Some(mut tokens) = r.get(&from) {
            if tokens.clear_for_transfer(tid, sid) == 0 {
                r.remove(&from);
            } else {
                r.insert(from, tokens);
            }
        }

        let mut tokens = r.get(&to).unwrap_or_default();
        tokens.0.entry(tid).or_default().insert(sid, None);
        r.insert(to, tokens);
    });
}

pub fn with<R>(f: impl FnOnce(&StableBTreeMap<Principal, OwnerTokens, Memory>) -> R) -> R {
    OWNER_TOKENS.with(|r| f(&r.borrow()))
}

pub fn with_mut<R>(f: impl FnOnce(&mut StableBTreeMap<Principal, OwnerTokens, Memory>) -> R) -> R {
    OWNER_TOKENS.with(|r| f(&mut r.borrow_mut()))
}
