use crate::utils::{sha3_256, Secret};
use crate::{
    is_authenticated, is_controller, memory, CreateTokenArg, SecretArg, TokenId,
    UpdateCollectionArg, UpdateTokenArg,
};
use candid::{Nat, Principal};
use serde_bytes::ByteBuf;
use std::collections::BTreeSet;

#[ic_cdk::update(guard = "is_controller")]
pub fn set_minters(args: BTreeSet<Principal>) -> Result<(), String> {
    let now = ic_cdk::api::time() / 1_000_000_000;
    memory::collection::with_mut(|r| {
        r.updated_at = now;
        r.minters = args;
    });
    Ok(())
}

#[ic_cdk::update(guard = "is_controller")]
pub fn set_managers(args: BTreeSet<Principal>) -> Result<(), String> {
    let now = ic_cdk::api::time() / 1_000_000_000;
    memory::collection::with_mut(|r| {
        r.updated_at = now;
        r.managers = args;
    });
    Ok(())
}

#[ic_cdk::update(guard = "is_authenticated")]
pub fn update_collection(args: UpdateCollectionArg) -> Result<(), String> {
    let caller = ic_cdk::caller();

    memory::collection::with(|c| {
        if !c.managers.contains(&caller) {
            ic_cdk::trap("caller is not a manager");
        }

        if let Some(supply_cap) = args.supply_cap {
            if supply_cap >= c.supply_cap.unwrap_or(0) {
                ic_cdk::trap("supply cap can not be increased");
            }
        }
    });

    let now = ic_cdk::api::time() / 1_000_000_000;
    memory::collection::with_mut(|r| {
        r.updated_at = now;

        if let Some(name) = args.name {
            r.name = name;
        }
        if let Some(val) = args.description {
            r.description = Some(val);
        }
        if let Some(val) = args.logo {
            r.logo = Some(val);
        }
        if let Some(val) = args.assets_origin {
            r.assets_origin = Some(val);
        }
        if let Some(val) = args.supply_cap {
            r.supply_cap = Some(val);
        }

        if let Some(val) = args.max_query_batch_size {
            r.settings.max_query_batch_size = val;
        }
        if let Some(val) = args.max_update_batch_size {
            r.settings.max_update_batch_size = val;
        }
        if let Some(val) = args.default_take_value {
            r.settings.default_take_value = val;
        }
        if let Some(val) = args.max_take_value {
            r.settings.max_take_value = val;
        }
        if let Some(val) = args.max_memo_size {
            r.settings.max_memo_size = val;
        }
        if let Some(val) = args.atomic_batch_transfers {
            r.settings.atomic_batch_transfers = val;
        }
        if let Some(val) = args.tx_window {
            r.settings.tx_window = val;
        }
        if let Some(val) = args.permitted_drift {
            r.settings.permitted_drift = val;
        }
        if let Some(val) = args.max_approvals_per_token_or_collection {
            r.settings.max_approvals_per_token_or_collection = val;
        }
        if let Some(val) = args.max_revoke_approvals {
            r.settings.max_revoke_approvals = val;
        }
    });

    Ok(())
}

#[ic_cdk::update(guard = "is_authenticated")]
pub fn secret(args: SecretArg) -> Result<ByteBuf, String> {
    let caller = ic_cdk::caller();

    memory::collection::with(|c| {
        if !c.managers.contains(&caller) {
            ic_cdk::trap("caller is not a manager");
        }
    });
    let ts = ic_cdk::api::time() / 1_000_000_000;
    memory::secret::with_secret(|secret| Ok(ByteBuf::from(args.secret(secret, ts))))
}

#[ic_cdk::update(guard = "is_authenticated")]
pub fn create_token(args: CreateTokenArg) -> Result<Nat, String> {
    let caller = ic_cdk::caller();
    memory::collection::with(|c| {
        if !c.managers.contains(&caller) {
            ic_cdk::trap(&format!("caller {caller} is not a manager"));
        }

        if let Some(supply_cap) = c.supply_cap {
            if c.total_supply >= supply_cap {
                ic_cdk::trap("supply cap reached");
            }
        }
    });

    let now = ic_cdk::api::time() / 1_000_000_000;
    let hash = sha3_256(&args.asset_content);
    create_token_(args, hash, now)
}

#[ic_cdk::update(guard = "is_authenticated")]
pub fn create_token_by_secret(args: CreateTokenArg) -> Result<Nat, String> {
    let caller = ic_cdk::caller();
    if caller != args.author {
        ic_cdk::trap("caller is not the author");
    }

    let challenge_data = args
        .challenge
        .as_ref()
        .unwrap_or_else(|| ic_cdk::trap("challenge is required"));

    memory::collection::with(|c| {
        if let Some(supply_cap) = c.supply_cap {
            if c.total_supply >= supply_cap {
                ic_cdk::trap("supply cap reached");
            }
        }
    });

    let now = ic_cdk::api::time() / 1_000_000_000;
    let expire_at = now - 60 * 10;
    let hash = sha3_256(&args.asset_content);
    memory::secret::with_secret(|secret| {
        SecretArg {
            author: caller,
            asset_hash: hash,
        }
        .verify(secret, expire_at, challenge_data)
    })?;

    create_token_(args, hash, now)
}

#[ic_cdk::update(guard = "is_authenticated")]
pub fn update_token(args: UpdateTokenArg) -> Result<(), String> {
    let caller = ic_cdk::caller();

    let id = TokenId::from(&args.id);
    let mut token = memory::token::with(|r| r.get(id.token_index() as u64)).unwrap_or_else(|| {
        ic_cdk::trap("token not found");
    });

    memory::collection::with(|c| {
        if !c.managers.contains(&caller) && token.author != caller {
            ic_cdk::trap("caller is not a manager or author");
        }
    });

    //    if token.total_supply > 0 {
    //        ic_cdk::trap("token has been minted, can not be updated");
    //    }

    if let Some(supply_cap) = args.supply_cap {
        if supply_cap >= token.supply_cap.unwrap_or(0) {
            ic_cdk::trap("supply cap can not be increased");
        }
    }

    let now = ic_cdk::api::time() / 1_000_000_000;
    token.updated_at = now;

    if let Some(name) = args.name {
        token.name = name;
    }
    if let Some(description) = args.description {
        token.description = Some(description);
    }
    if let Some(asset_name) = args.asset_name {
        token.asset_name = asset_name;
    }
    if let Some(asset_content_type) = args.asset_content_type {
        token.asset_content_type = asset_content_type;
    }

    if let Some(asset_content) = args.asset_content {
        let hash = sha3_256(&asset_content);
        memory::assets::with_mut(|r| {
            r.remove(&token.asset_hash);
            r.insert(hash, asset_content.to_vec());
        });
        token.asset_hash = hash;
    }

    if let Some(metadata) = args.metadata {
        token.metadata = metadata;
    }

    if let Some(supply_cap) = args.supply_cap {
        token.supply_cap = Some(supply_cap);
    }

    if let Some(author) = args.author {
        token.author = author;
    }

    memory::token::with_mut(|r| r.set(id.token_index() as u64, &token));

    Ok(())
}

fn create_token_(args: CreateTokenArg, hash: [u8; 32], now_sec: u64) -> Result<Nat, String> {
    memory::assets::with_mut(|r| {
        if r.contains_key(&hash) {
            return Err("asset already exists".to_string());
        }

        r.insert(hash, args.asset_content.to_vec());
        Ok::<(), String>(())
    })?;

    let id = memory::token::with_mut(|r| {
        let id = r.len() as u32 + 1;
        let token = memory::token::Token {
            id,
            name: args.name,
            description: args.description,
            asset_name: args.asset_name,
            asset_content_type: args.asset_content_type,
            asset_hash: hash,
            metadata: args.metadata,
            supply_cap: args.supply_cap,
            author: args.author,
            total_supply: 0,
            created_at: now_sec,
            updated_at: now_sec,
        };
        match r.push(&token) {
            Err(err) => Err(format!("failed to create token: {}", err)),
            Ok(_) => Ok(Nat::from(id)),
        }
    })?;

    memory::collection::with_mut(|r| {
        r.total_supply += 1;
        r.updated_at = now_sec;
    });

    Ok(id)
}
