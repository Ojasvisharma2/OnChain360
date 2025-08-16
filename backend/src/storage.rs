use candid::{CandidType, Decode, Encode};
use ic_stable_structures::{
    memory_manager::{MemoryId, MemoryManager, VirtualMemory},
    DefaultMemoryImpl, StableBTreeMap, StableCell,
};
use serde::de::DeserializeOwned;
use std::cell::RefCell;

type Mem = DefaultMemoryImpl;
type VMem = VirtualMemory<Mem>;
type Bytes = Vec<u8>;
type MapBytes = StableBTreeMap<u64, Bytes, VMem>;

thread_local! {
    pub static MEMORY_MANAGER: RefCell<MemoryManager<Mem>> =
        RefCell::new(MemoryManager::init(Mem::default()));

    pub static USERS: RefCell<MapBytes> = RefCell::new(
        MapBytes::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0)))
        )
    );

    pub static NEXT_USER_ID: RefCell<StableCell<u64, VMem>> = RefCell::new(
        StableCell::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(10))),
            0u64
        )
    );
}

pub fn next_user_id() -> u64 {
    NEXT_USER_ID.with(|cell| {
        let mut cell = cell.borrow_mut();
        let current = *cell.get();
        let next = current + 1;
        cell.set(next); // ✅ no &next, no expect
        current
    })
}

pub fn users_get<T: CandidType + DeserializeOwned>(user_id: u64) -> Option<T> {
    USERS.with(|m| {
        m.borrow()
            .get(&user_id)
            .and_then(|bytes| Decode!(&bytes, T).ok()) // ✅ &bytes is fine
    })
}

pub fn users_put<T: CandidType>(user_id: u64, user: &T) {
    USERS.with(|m| {
        let bytes = Encode!(user).expect("failed to encode user"); // ✅ convert to Vec<u8>
        m.borrow_mut().insert(user_id, bytes);
    });
}

pub fn users_remove(user_id: u64) -> Option<Bytes> {
    USERS.with(|m| m.borrow_mut().remove(&user_id))
}