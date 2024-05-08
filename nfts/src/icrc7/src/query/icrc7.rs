use candid::Nat;
use icrc_ledger_types::icrc1::account::Account;

use crate::{
    memory,
    types::{nat_to_u64, Metadata, TokenId},
};

#[ic_cdk::query]
pub fn icrc7_collection_metadata() -> Metadata {
    memory::collection::with(|c| c.metadata())
}

#[ic_cdk::query]
pub fn icrc7_symbol() -> String {
    memory::collection::with(|c| c.symbol.clone())
}

#[ic_cdk::query]
pub fn icrc7_name() -> String {
    memory::collection::with(|c| c.name.clone())
}

#[ic_cdk::query]
pub fn icrc7_description() -> Option<String> {
    memory::collection::with(|c| c.description.clone())
}

#[ic_cdk::query]
pub fn icrc7_logo() -> Option<String> {
    memory::collection::with(|c| c.logo.clone())
}

#[ic_cdk::query]
pub fn icrc7_total_supply() -> Nat {
    memory::collection::with(|c| c.total_supply.into())
}

#[ic_cdk::query]
pub fn icrc7_supply_cap() -> Option<Nat> {
    memory::collection::with(|c| c.supply_cap.map(Nat::from))
}

#[ic_cdk::query]
pub fn icrc7_max_query_batch_size() -> Option<Nat> {
    memory::collection::with(|c| Some(c.settings.max_query_batch_size.into()))
}

#[ic_cdk::query]
pub fn icrc7_max_update_batch_size() -> Option<Nat> {
    memory::collection::with(|c| Some(c.settings.max_update_batch_size.into()))
}

#[ic_cdk::query]
pub fn icrc7_default_take_value() -> Option<Nat> {
    memory::collection::with(|c| Some(c.settings.default_take_value.into()))
}

#[ic_cdk::query]
pub fn icrc7_max_take_value() -> Option<Nat> {
    memory::collection::with(|c| Some(c.settings.max_take_value.into()))
}

#[ic_cdk::query]
pub fn icrc7_max_memo_size() -> Option<Nat> {
    memory::collection::with(|c| Some(c.settings.max_memo_size.into()))
}

#[ic_cdk::query]
pub fn icrc7_atomic_batch_transfers() -> Option<bool> {
    memory::collection::with(|c| Some(c.settings.atomic_batch_transfers))
}
#[ic_cdk::query]
pub fn icrc7_tx_window() -> Option<Nat> {
    memory::collection::with(|c| Some(c.settings.tx_window.into()))
}

#[ic_cdk::query]
pub fn icrc7_permitted_drift() -> Option<Nat> {
    memory::collection::with(|c| Some(c.settings.permitted_drift.into()))
}

#[ic_cdk::query]
pub fn icrc7_token_metadata(token_ids: Vec<Nat>) -> Vec<Option<Metadata>> {
    if token_ids.is_empty() {
        return vec![];
    }

    let max_query_batch_size = memory::collection::with(|c| c.settings.max_query_batch_size);
    if token_ids.len() > max_query_batch_size as usize {
        ic_cdk::trap("exceeds max query batch size");
    }
    memory::token::with(|r| {
        token_ids
            .iter()
            .map(|id| {
                let id = TokenId::from(id);
                r.get(id.token_index() as u64).map(|t| t.metadata())
            })
            .collect()
    })
}

#[ic_cdk::query]
pub fn icrc7_owner_of(token_ids: Vec<Nat>) -> Vec<Option<Account>> {
    if token_ids.is_empty() {
        return vec![];
    }

    let max_query_batch_size = memory::collection::with(|c| c.settings.max_query_batch_size);
    if token_ids.len() > max_query_batch_size as usize {
        ic_cdk::trap("exceeds max query batch size");
    }

    memory::owners::with(|r| {
        token_ids
            .iter()
            .map(|id| {
                let id = TokenId::from(id);
                r.get(&id.0).and_then(|hs| {
                    hs.get(id.1).map(|h| Account {
                        owner: *h,
                        subaccount: None,
                    })
                })
            })
            .collect()
    })
}

#[ic_cdk::query]
pub fn icrc7_balance_of(accounts: Vec<Account>) -> Vec<Nat> {
    if accounts.is_empty() {
        return vec![];
    }

    let max_query_batch_size = memory::collection::with(|c| c.settings.max_query_batch_size);
    if accounts.len() > max_query_batch_size as usize {
        ic_cdk::trap("exceeds max query batch size");
    }

    memory::owner_tokens::with(|r| {
        let res: Vec<Nat> = accounts
            .into_iter()
            .map(|acc| {
                r.get(&acc.owner)
                    .map(|tokens| tokens.balance_of())
                    .unwrap_or(0u64)
            })
            .map(Nat::from)
            .collect();
        res
    })
}

#[ic_cdk::query]
pub fn icrc7_tokens(prev: Option<Nat>, take: Option<Nat>) -> Vec<Nat> {
    let take = memory::collection::take_value(take.as_ref().map(nat_to_u64));

    memory::token::with(|r| {
        let max_tid = r.len() as u32;
        let start_tid = if let Some(ref prev) = prev {
            TokenId::from(prev).0
        } else {
            1u32
        };
        let mut res: Vec<Nat> = Vec::with_capacity(take as usize);
        for tid in start_tid..=max_tid {
            res.push(Nat::from(TokenId(tid, 0).to_u64()));
            if res.len() as u16 >= take {
                return res;
            }
        }
        res
    })
}

#[ic_cdk::query]
pub fn icrc7_tokens_of(account: Account, prev: Option<Nat>, take: Option<Nat>) -> Vec<Nat> {
    let take = memory::collection::take_value(take.as_ref().map(nat_to_u64));

    memory::owner_tokens::with(|r| {
        r.get(&account.owner)
            .map(|tokens| {
                let TokenId(start_tid, mut start_sid) = if let Some(ref prev) = prev {
                    TokenId::from(prev).next()
                } else {
                    TokenId::MIN
                };

                let tids = tokens.token_ids();
                let mut res: Vec<Nat> = Vec::with_capacity(take as usize);
                for tid in tids {
                    if tid < start_tid {
                        continue;
                    }

                    if let Some(sids) = tokens.get_sids(tid) {
                        for sid in sids {
                            if sid < start_sid {
                                continue;
                            }
                            res.push(Nat::from(TokenId(tid, sid).to_u64()));
                            if res.len() as u16 >= take {
                                return res;
                            }
                        }
                    }
                    start_sid = 1;
                }
                res
            })
            .unwrap_or_default()
    })
}
