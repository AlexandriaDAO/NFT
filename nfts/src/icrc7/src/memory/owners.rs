use std::borrow::Cow;

use candid::Principal;
use ciborium::{from_reader, into_writer};
use ic_stable_structures::{storable::Bound, StableBTreeMap, Storable};
use serde::{Deserialize, Serialize};

use crate::{types::icrc37::TransferFromError, TransferError};

use super::{Memory, OWNERS};

#[derive(Clone, Deserialize, Serialize)]
pub struct Owners(pub Vec<Principal>);

impl Storable for Owners {
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

impl Owners {
    pub fn total(&self) -> u32 {
        self.0.len() as u32
    }

    pub fn get(&self, sid: u32) -> Option<&Principal> {
        self.0.get(sid as usize)
    }

    pub fn is_holder(&self, sid: u32, account: &Principal) -> bool {
        self.0
            .get(sid as usize)
            .map_or(false, |holder| holder == account)
    }

    pub fn append(&mut self, account: Principal) {
        self.0.push(account);
    }

    pub fn transfer_to(
        &mut self,
        from: &Principal,
        to: &Principal,
        sid: u32,
    ) -> Result<(), TransferError> {
        let holder = self
            .0
            .get_mut(sid as usize)
            .ok_or(TransferError::NonExistingTokenId)?;
        if holder != from {
            return Err(TransferError::Unauthorized);
        }
        *holder = *to;
        Ok(())
    }

    pub fn transfer_from(
        &mut self,
        from: &Principal,
        to: &Principal,
        sid: u32,
    ) -> Result<(), TransferFromError> {
        let holder = self
            .0
            .get_mut(sid as usize)
            .ok_or(TransferFromError::NonExistingTokenId)?;
        if holder != from {
            return Err(TransferFromError::Unauthorized);
        }
        *holder = *to;
        Ok(())
    }
}

pub fn with<R>(f: impl FnOnce(&StableBTreeMap<u32, Owners, Memory>) -> R) -> R {
    OWNERS.with(|r| f(&r.borrow()))
}

pub fn with_mut<R>(f: impl FnOnce(&mut StableBTreeMap<u32, Owners, Memory>) -> R) -> R {
    OWNERS.with(|r| f(&mut r.borrow_mut()))
}
