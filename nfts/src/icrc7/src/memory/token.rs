use std::borrow::Cow;

use crate::types::Metadata;
use candid::Principal;
use ciborium::{from_reader, into_writer};
use ic_stable_structures::{storable::Bound, StableVec, Storable};
use icrc_ledger_types::icrc::generic_value::Value;
use serde::{Deserialize, Serialize};
use serde_bytes::ByteBuf;

use super::{Memory, TOKENS};

#[derive(Clone, Deserialize, Serialize)]
pub struct Token {
    pub id: u32,
    pub name: String,
    pub description: Option<String>,
    pub asset_name: String,
    pub asset_content_type: String,
    pub asset_hash: [u8; 32],
    pub metadata: Metadata,
    pub author: Principal,
    pub supply_cap: Option<u32>,
    pub total_supply: u32,
    pub created_at: u64,
    pub updated_at: u64,
}

impl Storable for Token {
    const BOUND: Bound = Bound::Bounded {
        max_size: 100_000_000,
        is_fixed_size: false,
    };

    fn to_bytes(&self) -> Cow<[u8]> {
        let mut buf = vec![];
        into_writer(self, &mut buf).unwrap();
        Cow::Owned(buf)
    }

    fn from_bytes(bytes: Cow<'_, [u8]>) -> Self {
        from_reader(&bytes[..]).unwrap()
    }
}

impl Token {
    pub fn metadata(&self) -> Metadata {
        let mut res = self.metadata.clone();
        res.insert("icrc7:name".to_string(), Value::Text(self.name.clone()));
        if let Some(ref description) = self.description {
            res.insert(
                "icrc7:description".to_string(),
                Value::Text(description.clone()),
            );
        }
        res.insert(
            "asset_name".to_string(),
            Value::Text(self.asset_name.clone()),
        );
        res.insert(
            "asset_content_type".to_string(),
            Value::Text(self.asset_content_type.clone()),
        );
        res.insert(
            "asset_hash".to_string(),
            Value::Blob(ByteBuf::from(self.asset_hash.as_slice())),
        );
        res
    }
}

pub fn with<R>(f: impl FnOnce(&StableVec<Token, Memory>) -> R) -> R {
    TOKENS.with(|r| f(&r.borrow()))
}

pub fn with_mut<R>(f: impl FnOnce(&mut StableVec<Token, Memory>) -> R) -> R {
    TOKENS.with(|r| f(&mut r.borrow_mut()))
}
