use candid::{CandidType, Deserialize};
use serde::Serialize;

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct User {
    pub id: u64,
    pub username: String,
    pub bio: Option<String>,
    pub followers: Vec<u64>,   // store user IDs
    pub following: Vec<u64>,
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct Post {
    pub id: u64,
    pub author_id: u64,
    pub content: String,
    pub likes: Vec<u64>,       // user IDs who liked
    pub comments: Vec<u64>,    // comment IDs
    pub timestamp: u64,
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct Comment {
    pub id: u64,
    pub post_id: u64,
    pub author_id: u64,
    pub content: String,
    pub timestamp: u64,
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct Notification {
    pub id: u64,
    pub user_id: u64,          // who receives
    pub message: String,
    pub read: bool,
    pub timestamp: u64,
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct DirectMessage {
    pub id: u64,
    pub sender_id: u64,
    pub receiver_id: u64,
    pub content: String,
    pub timestamp: u64,
}


