use candid::Nat;

use crate::{
    guards::Validate,
    is_authenticated,
    memory::{self, owners::Owners},
    types::{
        icrc7::{TransferArg, TransferError, TransferResult},
        MintArg, MintError, MintResult, TokenId,
    },
};

#[ic_cdk::update(guard = "is_authenticated")]
pub fn icrc7_transfer(args: Vec<TransferArg>) -> Vec<Option<TransferResult>> {
    if args.is_empty() {
        ic_cdk::trap("no transfer args provided")
    }

    let settings = memory::collection::with(|c| c.settings.clone());

    if args.len() > settings.max_update_batch_size as usize {
        ic_cdk::trap("exceeds max update batch size");
    }

    let caller = ic_cdk::caller();
    let now = ic_cdk::api::time();
    if settings.atomic_batch_transfers && args.len() > 1 {
        if let Some(err) = args
            .iter()
            .find_map(|arg| arg.validate(now, &caller, &settings).err())
        {
            ic_cdk::trap(format!("invalid transfer args: {:?}", err).as_str())
        }

        if let Err(err) = memory::owners::with(|r| {
            for arg in &args {
                let id = TokenId::from(&arg.token_id);
                match r.get(&id.0) {
                    None => return Err(TransferError::NonExistingTokenId),
                    Some(ref holders) => {
                        if !holders.is_holder(id.1, &caller) {
                            return Err(TransferError::Unauthorized);
                        }
                    }
                }
            }
            Ok(())
        }) {
            ic_cdk::trap(format!("invalid transfer args: {:?}", err).as_str())
        }
    }

    memory::owners::with_mut(|r| {
        let mut res: Vec<Option<TransferResult>> = vec![None; args.len()];
        for (index, arg) in args.iter().enumerate() {
            if let Err(err) = arg.validate(now, &caller, &settings) {
                res[index] = Some(Err(err));
                continue;
            }

            let id = TokenId::from(&arg.token_id);
            match r.get(&id.0) {
                None => {
                    res[index] = Some(Err(TransferError::NonExistingTokenId));
                }
                Some(mut holders) => match holders.transfer_to(&caller, &arg.to.owner, id.1) {
                    Ok(_) => {
                        let tx_log = memory::transaction::Transaction::transfer(
                            now / 1_000_000_000,
                            id.to_u64(),
                            caller,
                            arg.to.owner,
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
                                res[index] = Some(Err(TransferError::GenericBatchError {
                                    error_code: Nat::from(0u64),
                                    message: err,
                                }));
                                // break up when append log failed.
                                return res;
                            }
                        }
                    }
                    Err(err) => {
                        res[index] = Some(Err(err));
                    }
                },
            }
        }

        res
    })
}

#[ic_cdk::update(guard = "is_authenticated")]
pub fn mint(args: MintArg) -> MintResult {
    let caller = ic_cdk::caller();
    if !memory::collection::with(|c| c.minters.contains(&caller)) {
        ic_cdk::trap("caller is not a minter");
    }

    if args.holders.is_empty() {
        ic_cdk::trap("no mint holders provided")
    }

    let settings = memory::collection::with(|c| c.settings.clone());
    if args.holders.len() > settings.max_update_batch_size as usize {
        ic_cdk::trap("exceeds max update batch size");
    }

    let id = TokenId::from(&args.token_id);
    let metadata = memory::token::with(|r| {
        if let Some(token) = r.get(id.token_index() as u64) {
            if let Some(supply_cap) = token.supply_cap {
                if token.total_supply.saturating_add(args.holders.len() as u32) >= supply_cap {
                    return Err(MintError::SupplyCapReached);
                }
            }

            Ok(token.metadata())
        } else {
            Err(MintError::NonExistingTokenId)
        }
    })?;

    let now_sec = ic_cdk::api::time() / 1_000_000_000;
    memory::owners::with_mut(|r| {
        let mut holders = match r.get(&id.0) {
            None => Owners(vec![]),
            Some(holders) => holders,
        };
        let mut block_idx = 0u64;
        let added_holders = args.holders.len() as u32;
        for holder in args.holders {
            holders.append(holder);

            let tx_log = memory::transaction::Transaction::mint(
                now_sec,
                id.to_u64(),
                Some(caller),
                holder,
                metadata.clone(),
                None,
            );

            match memory::transaction::append(&tx_log) {
                Ok(idx) => block_idx = idx,
                Err(err) => {
                    // break up when append log failed.
                    return Err(MintError::GenericBatchError {
                        error_code: Nat::from(0u64),
                        message: err,
                    });
                }
            }
        }
        r.insert(id.0, holders);
        memory::token::with_mut(|r| {
            let idx = id.token_index() as u64;
            if let Some(mut token) = r.get(idx) {
                token.total_supply += added_holders;
                token.updated_at = now_sec;
                r.set(idx, &token);
            }
        });

        Ok(Nat::from(block_idx))
    })
}
