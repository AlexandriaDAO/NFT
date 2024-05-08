use candid::Principal;
use ic_stable_structures::{
    memory_manager::{MemoryId, MemoryManager, VirtualMemory},
    DefaultMemoryImpl, StableBTreeMap, StableCell, StableLog, StableVec,
};
use serde::{Deserialize, Serialize};
use std::cell::RefCell;

pub mod approvals;
pub mod assets;
pub mod collection;
pub mod owner_tokens;
pub mod owners;
pub mod secret;
pub mod token;
pub mod transaction;

use collection::Collection;
use owners::Owners;
use token::Token;

use self::{approvals::Approvals, owner_tokens::OwnerTokens, transaction::Transaction};

pub type Memory = VirtualMemory<DefaultMemoryImpl>;

thread_local! {
    static SECRET: RefCell<[u8; 32]> = const { RefCell::new([0; 32]) };

    static HEAP: RefCell<Collection> = RefCell::new(Collection::default());

    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

    static COLLECTION: RefCell<StableCell<Collection, Memory>> = RefCell::new(
        StableCell::init(
            MEMORY_MANAGER.with_borrow(|m| m.get(MemoryId::new(0))),
            Collection::default()
        ).unwrap());

    static TOKENS: RefCell<StableVec<Token, Memory>> = RefCell::new(
        StableVec::init(
            MEMORY_MANAGER.with_borrow(|m| m.get(MemoryId::new(1)))
        ).unwrap());

    static OWNERS: RefCell<StableBTreeMap<u32, Owners, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with_borrow(|m| m.get(MemoryId::new(2))),
        ));

    static OWNER_TOKENS: RefCell<StableBTreeMap<Principal, OwnerTokens, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with_borrow(|m| m.get(MemoryId::new(3))),
        ));

    static OWNER_APPROVALS: RefCell<StableBTreeMap<Principal, Approvals, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with_borrow(|m| m.get(MemoryId::new(4))),
        ));

    static ASSETS: RefCell<StableBTreeMap<[u8; 32], Vec<u8>, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with_borrow(|m| m.get(MemoryId::new(5))),
        ));

    static TRANSACTIONS: RefCell<StableLog<Transaction, Memory, Memory>> = RefCell::new(
        StableLog::init(
            MEMORY_MANAGER.with_borrow(|m| m.get(MemoryId::new(5))),
            MEMORY_MANAGER.with_borrow(|m| m.get(MemoryId::new(6))),
        ).unwrap());
}

#[derive(Clone, Default, Deserialize, Serialize)]
pub struct Settings {
    pub max_query_batch_size: u16,
    pub max_update_batch_size: u16,
    pub default_take_value: u16,
    pub max_take_value: u16,
    pub max_memo_size: u16,
    pub atomic_batch_transfers: bool,
    pub tx_window: u64,
    pub permitted_drift: u64,
    pub max_approvals_per_token_or_collection: u16,
    pub max_revoke_approvals: u16,
}
