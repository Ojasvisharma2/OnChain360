mod storage;
mod auth;
mod models;
mod utils;

use ic_cdk_macros::*;
use candid::{export_service, CandidType};
use serde::Deserialize;

use storage::*;
use auth::{AuthResult, signup, login, request_password_reset, reset_password};
use models::*;
use utils::now_seconds;

#[init]
fn init() {
    // Initialize storage or state if needed
    ic_cdk::println!("Canister initialized!");
}

// Auth API
#[update]
pub fn signup_user(username: String, email: String, password: String, bio: Option<String>) 
    -> Result<User, AuthResult> 
{
    signup(username, email, password, bio)
}

#[update]
pub fn login_user(username: String, password: String) -> AuthResult {
    login(username, password)
}

#[update]
pub fn request_reset(email: String) -> Result<String, AuthResult> {
    request_password_reset(email)
}

#[update]
pub fn confirm_reset(email: String, code: String, new_password: String) -> AuthResult {
    reset_password(email, code, new_password)
}

// Users
#[query]
pub fn get_user(user_id: u64) -> Option<User> {
    users_get::<User>(user_id)
}

#[query]
pub fn list_users() -> Vec<User> {
    let max = storage::NEXT_USER_ID.with(|c| *c.borrow().get());
    (0..max).filter_map(|id| users_get::<User>(id)).collect()
}

#[update]
pub fn follow(follower_id: u64, followee_id: u64) -> bool {
    follow_user(follower_id, followee_id)
}

#[update]
pub fn unfollow(follower_id: u64, followee_id: u64) -> bool {
    unfollow_user(follower_id, followee_id)
}

// Posts
#[update]
pub fn create_post_api(author_id: u64, content: String) -> u64 {
    create_post(author_id, content, now_seconds())
}

#[query]
pub fn get_post_api(post_id: u64) -> Option<Post> {
    posts_get::<Post>(post_id)
}

#[update]
pub fn like_post_api(user_id: u64, post_id: u64) -> bool {
    like_post(user_id, post_id)
}

#[update]
pub fn unlike_post_api(user_id: u64, post_id: u64) -> bool {
    unlike_post(user_id, post_id)
}

#[query]
pub fn get_post_likes(post_id: u64) -> Vec<u64> {
    posts_get::<Post>(post_id).map(|p| p.likes).unwrap_or_default()
}

// Comments
#[update]
pub fn add_comment_api(post_id: u64, author_id: u64, content: String) -> u64 {
    add_comment(post_id, author_id, content, now_seconds())
}

#[query]
pub fn get_comment(comment_id: u64) -> Option<Comment> {
    comments_get::<Comment>(comment_id)
}

#[query]
pub fn list_post_comments(post_id: u64) -> Vec<Comment> {
    let max = storage::NEXT_COMMENT_ID.with(|c| *c.borrow().get());
    (0..max)
        .filter_map(|id| comments_get::<Comment>(id))
        .filter(|c| c.post_id == post_id)
        .collect()
}

// Notifications
#[update]
pub fn add_notification_api(user_id: u64, message: String) -> u64 {
    add_notification(user_id, message, now_seconds())
}

#[query]
pub fn get_notifications_api(user_id: u64) -> Vec<Notification> {
    list_notifications(user_id)
}

// Direct Messages
#[update]
pub fn send_dm_api(sender_id: u64, receiver_id: u64, content: String) -> u64 {
    send_dm(sender_id, receiver_id, content, now_seconds())
}

#[query]
pub fn get_dms_api(user_id: u64) -> Vec<DirectMessage> {
    list_dms(user_id)
}

#[query]
pub fn list_posts() -> Vec<Post> {
    let max = storage::NEXT_POST_ID.with(|c| *c.borrow().get());
    (0..max).filter_map(|id| posts_get::<Post>(id)).collect()
}

#[query]
pub fn list_user_posts(user_id: u64) -> Vec<Post> {
    let max = storage::NEXT_POST_ID.with(|c| *c.borrow().get());
    (0..max)
        .filter_map(|id| posts_get::<Post>(id))
        .filter(|p| p.author_id == user_id)
        .collect()
}

// Export Candid
ic_cdk::export_candid!();