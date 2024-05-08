use candid::{Nat, Principal};

use crate::{
    guards::Validate,
    is_authenticated, memory,
    types::{
        icrc37::{
            ApproveCollectionArg, ApproveCollectionError, ApproveCollectionResult, ApproveTokenArg,
            ApproveTokenError, ApproveTokenResult, RevokeCollectionApprovalArg,
            RevokeCollectionApprovalError, RevokeCollectionApprovalResult, RevokeTokenApprovalArg,
            RevokeTokenApprovalError, RevokeTokenApprovalResult, TransferFromArg,
            TransferFromError, TransferFromResult,
        },
        TokenId,
    },
};

#[ic_cdk::update(guard = "is_authenticated")]
pub fn icrc37_approve_tokens(args: Vec<ApproveTokenArg>) -> Vec<Option<ApproveTokenResult>> {
    let caller = ic_cdk::caller();

    if args.is_empty() {
        ic_cdk::trap("no ApproveTokenArgs provided")
    }

    let settings = memory::collection::with(|c| c.settings.clone());
    if args.len() > settings.max_update_batch_size as usize {
        ic_cdk::trap("exceeds max update batch size");
    }

    memory::owner_tokens::with_mut(|r| {
        let mut res: Vec<Option<ApproveTokenResult>> = vec![None; args.len()];
        let now = ic_cdk::api::time();
        match r.get(&caller) {
            None => {
                res.fill(Some(Err(ApproveTokenError::Unauthorized)));
            }
            Some(mut tokens) => {
                for (index, arg) in args.iter().enumerate() {
                    if let Err(err) = arg.validate(now, &caller, &settings) {
                        res[index] = Some(Err(err));
                        continue;
                    }

                    let id = TokenId::from(&arg.token_id);
                    match tokens.insert_approvals(
                        settings.max_approvals_per_token_or_collection,
                        id.0,
                        id.1,
                        arg.approval_info.spender.owner,
                        arg.approval_info.created_at_time.unwrap_or_default() / 1_000_000_000,
                        arg.approval_info.expires_at.unwrap_or_default() / 1_000_000_000,
                    ) {
                        Ok(_) => {
                            let tx_log = memory::transaction::Transaction::approve(
                                now / 1_000_000_000,
                                id.to_u64(),
                                caller,
                                arg.approval_info.spender.owner,
                                arg.approval_info.expires_at,
                                arg.approval_info.memo.to_owned(),
                            );

                            match memory::transaction::append(&tx_log) {
                                Ok(idx) => {
                                    res[index] = Some(Ok(Nat::from(idx)));
                                }
                                Err(err) => {
                                    res[index] = Some(Err(ApproveTokenError::GenericBatchError {
                                        error_code: Nat::from(0u64),
                                        message: err,
                                    }));
                                    r.insert(caller, tokens);
                                    // break up when append log failed.
                                    return res;
                                }
                            }
                        }
                        Err(err) => {
                            res[index] = Some(Err(err));
                        }
                    }
                }

                r.insert(caller, tokens);
            }
        }
        res
    })
}

#[ic_cdk::update(guard = "is_authenticated")]
pub fn icrc37_approve_collection(
    args: Vec<ApproveCollectionArg>,
) -> Vec<Option<ApproveCollectionResult>> {
    let caller = ic_cdk::caller();

    if args.is_empty() {
        ic_cdk::trap("no ApproveCollectionArg provided")
    }

    let settings = memory::collection::with(|c| c.settings.clone());
    if args.len() > settings.max_update_batch_size as usize {
        ic_cdk::trap("exceeds max update batch size");
    }

    memory::approvals::with_mut(|r| {
        let mut res: Vec<Option<ApproveCollectionResult>> = vec![None; args.len()];
        let now = ic_cdk::api::time();
        let mut approvals = r.get(&caller).unwrap_or_default();
        let mut total = approvals.total();
        if total >= settings.max_approvals_per_token_or_collection as u32 {
            res.fill(Some(Err(ApproveCollectionError::GenericBatchError {
                error_code: Nat::from(0u64),
                message: "exceeds the maximum number of approvals".to_string(),
            })));
        } else {
            for (index, arg) in args.iter().enumerate() {
                if let Err(err) = arg.validate(now, &caller, &settings) {
                    res[index] = Some(Err(err));
                    continue;
                }
                if total >= settings.max_approvals_per_token_or_collection as u32 {
                    res[index] = Some(Err(ApproveCollectionError::GenericBatchError {
                        error_code: Nat::from(0u64),
                        message: "exceeds the maximum number of approvals".to_string(),
                    }));
                    continue;
                }

                approvals.insert(
                    arg.approval_info.spender.owner,
                    arg.approval_info.created_at_time.unwrap_or_default() / 1_000_000_000,
                    arg.approval_info.expires_at.unwrap_or_default() / 1_000_000_000,
                );
                total += 1;

                let tx_log = memory::transaction::Transaction::approve_collection(
                    now / 1_000_000_000,
                    caller,
                    arg.approval_info.spender.owner,
                    arg.approval_info.expires_at,
                    arg.approval_info.memo.to_owned(),
                );

                match memory::transaction::append(&tx_log) {
                    Ok(idx) => {
                        res[index] = Some(Ok(Nat::from(idx)));
                    }
                    Err(err) => {
                        res[index] = Some(Err(ApproveCollectionError::GenericBatchError {
                            error_code: Nat::from(0u64),
                            message: err,
                        }));
                        r.insert(caller, approvals);
                        // break up when append log failed.
                        return res;
                    }
                }
            }

            r.insert(caller, approvals);
        }

        res
    })
}

#[ic_cdk::update(guard = "is_authenticated")]
pub fn icrc37_revoke_token_approvals(
    args: Vec<RevokeTokenApprovalArg>,
) -> Vec<Option<RevokeTokenApprovalResult>> {
    let caller = ic_cdk::caller();

    if args.is_empty() {
        ic_cdk::trap("no ApproveCollectionArg provided")
    }

    let settings = memory::collection::with(|c| c.settings.clone());
    if args.len() > settings.max_revoke_approvals as usize {
        ic_cdk::trap("exceeds max revoke approvals");
    }

    memory::owner_tokens::with_mut(|r| {
        let mut res: Vec<Option<RevokeTokenApprovalResult>> = vec![None; args.len()];
        let now = ic_cdk::api::time();
        match r.get(&caller) {
            None => {
                res.fill(Some(Err(RevokeTokenApprovalError::Unauthorized)));
            }
            Some(mut tokens) => {
                for (index, arg) in args.iter().enumerate() {
                    if let Err(err) = arg.validate(now, &caller, &settings) {
                        res[index] = Some(Err(err));
                        continue;
                    }

                    let id = TokenId::from(&arg.token_id);
                    let spender = arg.spender.map(|s| s.owner);
                    match tokens.revoke(id.0, id.1, spender) {
                        Err(err) => {
                            res[index] = Some(Err(err));
                        }
                        Ok(_) => {
                            let tx_log = memory::transaction::Transaction::revoke(
                                now / 1_000_000_000,
                                id.to_u64(),
                                caller,
                                spender,
                                arg.memo.to_owned(),
                            );

                            match memory::transaction::append(&tx_log) {
                                Ok(idx) => {
                                    res[index] = Some(Ok(Nat::from(idx)));
                                }
                                Err(err) => {
                                    res[index] =
                                        Some(Err(RevokeTokenApprovalError::GenericBatchError {
                                            error_code: Nat::from(0u64),
                                            message: err,
                                        }));
                                    r.insert(caller, tokens);
                                    // break up when append log failed.
                                    return res;
                                }
                            }
                        }
                    }
                }

                r.insert(caller, tokens);
            }
        }

        res
    })
}

#[ic_cdk::update(guard = "is_authenticated")]
pub fn icrc37_revoke_collection_approvals(
    args: Vec<RevokeCollectionApprovalArg>,
) -> Vec<Option<RevokeCollectionApprovalResult>> {
    let caller = ic_cdk::caller();

    if args.is_empty() {
        ic_cdk::trap("no RevokeCollectionApprovalArg provided")
    }

    let settings = memory::collection::with(|c| c.settings.clone());
    if args.len() > settings.max_revoke_approvals as usize {
        ic_cdk::trap("exceeds max revoke approvals");
    }
    let now = ic_cdk::api::time();
    let mut idxs: Vec<usize> = Vec::new();
    let mut spenders: Vec<Option<Principal>> = Vec::new();
    let mut res: Vec<Option<RevokeCollectionApprovalResult>> = vec![None; spenders.len()];
    for (i, arg) in args.iter().enumerate() {
        if let Err(err) = arg.validate(now, &caller, &settings) {
            res[i] = Some(Err(err));
            continue;
        }

        idxs.push(i);
        spenders.push(arg.spender.map(|s| s.owner));
    }

    let res2 = memory::approvals::revoke(&caller, &spenders);
    for (i, idx) in idxs.into_iter().enumerate() {
        match res2[i] {
            Some(ref val) => {
                res[idx] = Some(val.to_owned());
            }
            None => {
                let tx_log = memory::transaction::Transaction::revoke_collection(
                    now / 1_000_000_000,
                    caller,
                    spenders[i],
                    args[idx].memo.to_owned(),
                );

                match memory::transaction::append(&tx_log) {
                    Ok(block_idx) => {
                        res[idx] = Some(Ok(Nat::from(block_idx)));
                    }
                    Err(err) => {
                        res[idx] = Some(Err(RevokeCollectionApprovalError::GenericBatchError {
                            error_code: Nat::from(0u64),
                            message: err,
                        }));
                    }
                }
            }
        }
    }

    res
}

#[ic_cdk::update(guard = "is_authenticated")]
pub fn icrc37_transfer_from(args: Vec<TransferFromArg>) -> Vec<Option<TransferFromResult>> {
    if args.is_empty() {
        ic_cdk::trap("no transfer args provided")
    }

    let settings = memory::collection::with(|c| c.settings.clone());

    if args.len() > settings.max_update_batch_size as usize {
        ic_cdk::trap("exceeds max update batch size");
    }

    let caller = ic_cdk::caller();
    let now = ic_cdk::api::time();
    let now_sec = now / 1_000_000_000;
    if settings.atomic_batch_transfers && args.len() > 1 {
        if let Some(err) = args
            .iter()
            .find_map(|arg| arg.validate(now, &caller, &settings).err())
        {
            ic_cdk::trap(format!("invalid transfer from args: {:?}", err).as_str())
        }

        let query: Vec<(TokenId, &Principal)> = args
            .iter()
            .map(|arg| (TokenId::from(&arg.token_id), &arg.from.owner))
            .collect();

        let query = memory::approvals::find_unapproved(&caller, &query, now_sec);

        if let Err(from) = memory::owner_tokens::all_is_approved(&caller, &query, now_sec) {
            ic_cdk::trap(
                format!("(from: {}, spender: {}) are not approved", from, caller).as_str(),
            );
        }
    }

    memory::owners::with_mut(|r| {
        let mut res: Vec<Option<TransferFromResult>> = vec![None; args.len()];
        for (index, arg) in args.iter().enumerate() {
            if let Err(err) = arg.validate(now, &caller, &settings) {
                res[index] = Some(Err(err));
                continue;
            }

            let id = TokenId::from(&arg.token_id);
            if !memory::approvals::is_approved(&arg.from.owner, &caller, now_sec)
                && !memory::owner_tokens::is_approved(&arg.from.owner, &caller, id.0, id.1, now_sec)
            {
                res[index] = Some(Err(TransferFromError::Unauthorized));
                continue;
            }

            match r.get(&id.0) {
                None => {
                    res[index] = Some(Err(TransferFromError::NonExistingTokenId));
                }
                Some(mut holders) => {
                    match holders.transfer_from(&arg.from.owner, &arg.to.owner, id.1) {
                        Ok(_) => {
                            let tx_log = memory::transaction::Transaction::transfer_from(
                                now / 1_000_000_000,
                                id.to_u64(),
                                arg.from.owner,
                                arg.to.owner,
                                caller,
                                arg.memo.clone(),
                            );

                            match memory::transaction::append(&tx_log) {
                                Ok(idx) => {
                                    res[index] = Some(Ok(Nat::from(idx)));
                                    r.insert(id.0, holders);
                                    memory::owner_tokens::update_for_transfer(
                                        caller,
                                        arg.to.owner,
                                        id.0,
                                        id.1,
                                    );
                                }
                                Err(err) => {
                                    res[index] = Some(Err(TransferFromError::GenericBatchError {
                                        error_code: Nat::from(0u64),
                                        message: err,
                                    }));
                                    return res;
                                }
                            }
                        }
                        Err(err) => {
                            res[index] = Some(Err(err));
                        }
                    }
                }
            }
        }

        res
    })
}
