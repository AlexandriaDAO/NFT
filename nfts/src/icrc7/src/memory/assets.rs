use super::{Memory, ASSETS};
use ic_stable_structures::StableBTreeMap;

pub fn total() -> u64 {
    ASSETS.with(|r| r.borrow().len())
}

pub fn with<R>(f: impl FnOnce(&StableBTreeMap<[u8; 32], Vec<u8>, Memory>) -> R) -> R {
    ASSETS.with(|r| f(&r.borrow()))
}

pub fn with_mut<R>(f: impl FnOnce(&mut StableBTreeMap<[u8; 32], Vec<u8>, Memory>) -> R) -> R {
    ASSETS.with(|r| f(&mut r.borrow_mut()))
}
