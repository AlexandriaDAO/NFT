use std::{borrow::Cow, collections::BTreeSet};

use candid::Principal;
use ciborium::{from_reader, into_writer};
use ic_stable_structures::{storable::Bound, Storable};
use serde::{Deserialize, Serialize};

use crate::types::Metadata;
use icrc_ledger_types::icrc::generic_value::Value;

use super::{Settings, COLLECTION, HEAP};

#[derive(Clone, Default, Deserialize, Serialize)]
pub struct Collection {
    pub symbol: String,
    pub name: String,
    pub description: Option<String>,
    pub logo: Option<String>,
    pub assets_origin: Option<String>, // for example, "https://assets.panda.fans"
    pub total_supply: u64,
    pub supply_cap: Option<u64>,
    pub created_at: u64,
    pub updated_at: u64,

    pub minters: BTreeSet<Principal>,
    pub managers: BTreeSet<Principal>,
    pub settings: Settings,
}

impl Storable for Collection {
    const BOUND: Bound = Bound::Unbounded;

    fn to_bytes(&self) -> Cow<[u8]> {
        let mut buf = vec![];
        into_writer(self, &mut buf).expect("failed to encode Collection data");
        Cow::Owned(buf)
    }

    fn from_bytes(bytes: Cow<'_, [u8]>) -> Self {
        from_reader(&bytes[..]).expect("failed to decode Collection data")
    }
}

impl Collection {
    pub fn metadata(&self) -> Metadata {
        let mut res = Metadata::new();
        res.insert("icrc7:symbol".to_string(), Value::Text(self.symbol.clone()));
        res.insert("icrc7:name".to_string(), Value::Text(self.name.clone()));
        if let Some(ref description) = self.description {
            res.insert(
                "icrc7:description".to_string(),
                Value::Text(description.clone()),
            );
        }
        if let Some(ref logo) = self.logo {
            res.insert("icrc7:logo".to_string(), Value::Text(logo.clone()));
        }
        res.insert(
            "icrc7:total_supply".to_string(),
            Value::Nat(self.total_supply.into()),
        );
        if let Some(supply_cap) = self.supply_cap {
            res.insert(
                "icrc7:supply_cap".to_string(),
                Value::Nat(supply_cap.into()),
            );
        }
        res
    }

    pub fn icrc37_metadata(&self) -> Metadata {
        let mut res = Metadata::new();
        if self.settings.max_approvals_per_token_or_collection > 0 {
            res.insert(
                "icrc37:max_approvals_per_token_or_collection".to_string(),
                Value::Nat((self.settings.max_approvals_per_token_or_collection as u64).into()),
            );
        }
        if self.settings.max_revoke_approvals > 0 {
            res.insert(
                "icrc37:max_revoke_approvals".to_string(),
                Value::Nat((self.settings.max_revoke_approvals as u64).into()),
            );
        }
        res
    }
}

pub fn take_value(take: Option<u64>) -> u16 {
    with(|c| {
        take.map_or(c.settings.default_take_value, |t| {
            t.min(c.settings.max_take_value as u64) as u16
        })
    })
}

pub fn with<R>(f: impl FnOnce(&Collection) -> R) -> R {
    HEAP.with(|r| f(&r.borrow()))
}

pub fn with_mut<R>(f: impl FnOnce(&mut Collection) -> R) -> R {
    HEAP.with(|r| f(&mut r.borrow_mut()))
}

pub fn load() {
    COLLECTION.with(|r| {
        HEAP.with(|h| {
            *h.borrow_mut() = r.borrow().get().clone();
        });
    });
}

pub fn save() {
    HEAP.with(|h| {
        COLLECTION.with(|r| {
            r.borrow_mut()
                .set(h.borrow().clone())
                .expect("failed to set COLLECTION data");
        });
    });
}
