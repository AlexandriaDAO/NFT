use candid::{Nat, Principal};
use icrc_ledger_types::icrc1::account::Account;

use crate::{
    memory,
    types::{
        icrc37::{CollectionApproval, IsApprovedArg, TokenApproval},
        nat_to_u64, Metadata, TokenId,
    },
    ANONYMOUS,
};

#[ic_cdk::query]
pub fn icrc37_metadata() -> Metadata {
    memory::collection::with(|c| c.icrc37_metadata())
}

#[ic_cdk::query]
pub fn icrc37_max_approvals_per_token_or_collection() -> Option<Nat> {
    memory::collection::with(|c| Some(Nat::from(c.settings.max_approvals_per_token_or_collection)))
}

#[ic_cdk::query]
pub fn icrc37_max_revoke_approvals() -> Option<Nat> {
    memory::collection::with(|c| Some(Nat::from(c.settings.max_revoke_approvals)))
}

#[ic_cdk::query]
pub fn icrc37_is_approved(args: Vec<IsApprovedArg>) -> Vec<bool> {
    if args.is_empty() {
        return vec![];
    }

    let max_query_batch_size = memory::collection::with(|c| c.settings.max_query_batch_size);
    if args.len() > max_query_batch_size as usize {
        ic_cdk::trap("exceeds max query batch size");
    }
    let caller = ic_cdk::caller();
    if caller == ANONYMOUS {
        return vec![false; args.len()];
    }

    let now_sec = ic_cdk::api::time() / 1000_000_000;
    let spenders: Vec<&Principal> = args.iter().map(|a| &a.spender.owner).collect();
    let mut res = memory::approvals::spenders_is_approved(&caller, &spenders, now_sec);
    let mut query_idx: Vec<usize> = Vec::new();
    let mut query: Vec<(TokenId, &Principal)> = Vec::new();
    for (i, a) in args.iter().enumerate() {
        if !res[i] {
            query_idx.push(i);
            query.push((TokenId::from(&a.token_id), &a.spender.owner));
        }
    }
    let res2 = memory::owner_tokens::spenders_is_approved(&caller, &query, now_sec);
    for (i, idx) in query_idx.into_iter().enumerate() {
        res[idx] = res2[i];
    }

    res
}

#[ic_cdk::query]
pub fn icrc37_get_token_approvals(
    token_id: Nat,
    prev: Option<TokenApproval>,
    take: Option<Nat>,
) -> Vec<TokenApproval> {
    let id = TokenId::from(&token_id);
    let take = memory::collection::take_value(take.as_ref().map(nat_to_u64));
    let holder = memory::owners::with(|r| r.get(&id.0).and_then(|hs| hs.get(id.1).cloned()));
    let holder = match holder {
        Some(h) => h,
        None => return vec![],
    };

    memory::owner_tokens::with(|r| {
        if let Some(tokens) = r.get(&holder) {
            if let Some(approvals) = tokens.get_approvals(id.0, id.1) {
                let prev = prev.map(|p| p.approval_info.spender.owner);
                let mut res: Vec<TokenApproval> = Vec::with_capacity(take as usize);
                for approval in approvals.iter() {
                    if let Some(ref prev) = prev {
                        if approval.0 <= prev {
                            continue;
                        }
                    }
                    res.push(TokenApproval {
                        token_id: token_id.clone(),
                        approval_info: memory::approvals::Approvals::to_info(approval),
                    });

                    if res.len() as u16 >= take {
                        return res;
                    }
                }
                return res;
            }
        }

        vec![]
    })
}

#[ic_cdk::query]
pub fn icrc37_get_collection_approvals(
    owner: Account,
    prev: Option<CollectionApproval>,
    take: Option<Nat>,
) -> Vec<CollectionApproval> {
    let take = memory::collection::take_value(take.as_ref().map(nat_to_u64));

    memory::approvals::with(|r| {
        if let Some(approvals) = r.get(&owner.owner) {
            let prev = prev.map(|p| p.spender.owner);
            let mut res: Vec<CollectionApproval> = Vec::with_capacity(take as usize);
            for approval in approvals.iter() {
                if let Some(ref prev) = prev {
                    if approval.0 <= prev {
                        continue;
                    }
                }
                res.push(memory::approvals::Approvals::to_info(approval));

                if res.len() as u16 >= take {
                    return res;
                }
            }
            return res;
        }

        vec![]
    })
}
