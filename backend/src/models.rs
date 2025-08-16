// backend/src/models.rs
use candid::CandidType;
use serde::{Deserialize, Serialize};

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct User {
    pub id: u64,
    pub username: String,
    pub bio: String,
    pub joined_at: u64, // nanos since epoch
}