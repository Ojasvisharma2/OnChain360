use candid::{candid_method};
use ic_cdk::api::time;
use ic_cdk_macros::*;
use ic_stable_structures::StableBTreeMap;
use std::cell::RefCell;

mod models;
mod utils;
use models::*;
use utils::*;

// ---------- STABLE STORAGE ---------- //


// ---------- USER FUNCTIONS ---------- //
#[update]
#[candid_method(update)]
fn create_user(id: u64, username: String, bio: Option<String>) {
    let user = User {
        id,
        username,
        bio,
        followers: vec![],
        following: vec![],
    };
    USERS.with(|users| {
        users.borrow_mut().insert(id, user);
    });
}

#[query]
#[candid_method(query)]
fn get_user(id: u64) -> Option<User> {
    USERS.with(|users| users.borrow().get(&id).cloned())
}

// ---------- POST FUNCTIONS ---------- //
#[update]
#[candid_method(update)]
fn create_post(id: u64, author_id: u64, content: String) {
    let post = Post {
        id,
        author_id,
        content,
        likes: vec![],
        comments: vec![],
        timestamp: now_seconds(),
    };
    POSTS.with(|posts| {
        posts.borrow_mut().insert(id, post);
    });
}

#[query]
#[candid_method(query)]
fn get_post(id: u64) -> Option<Post> {
    POSTS.with(|posts| posts.borrow().get(&id).cloned())
}

// ---------- COMMENT FUNCTIONS ---------- //
#[update]
#[candid_method(update)]
fn add_comment(id: u64, post_id: u64, author_id: u64, content: String) {
    let comment = Comment {
        id,
        post_id,
        author_id,
        content: content.clone(),
        timestamp: now_seconds(),
    };

    COMMENTS.with(|comments| {
        comments.borrow_mut().insert(id, comment);
    });

    POSTS.with(|posts| {
        if let Some(mut post) = posts.borrow().get(&post_id).cloned() {
            post.comments.push(id);
            posts.borrow_mut().insert(post_id, post);
        }
    });
}

#[query]
#[candid_method(query)]
fn get_comment(id: u64) -> Option<Comment> {
    COMMENTS.with(|comments| comments.borrow().get(&id).cloned())
}

// ---------- NOTIFICATION FUNCTIONS ---------- //
#[update]
#[candid_method(update)]
fn add_notification(id: u64, user_id: u64, message: String) {
    let notif = Notification {
        id,
        user_id,
        message,
        read: false,
        timestamp: now_seconds(),
    };
    NOTIFICATIONS.with(|n| {
        n.borrow_mut().insert(id, notif);
    });
}

#[query]
#[candid_method(query)]
fn get_notifications(user_id: u64) -> Vec<Notification> {
    NOTIFICATIONS.with(|n| {
        n.borrow()
            .iter()
            .filter(|(_, notif)| notif.user_id == user_id)
            .map(|(_, notif)| notif.clone())
            .collect()
    })
}

// ---------- DIRECT MESSAGES ---------- //
#[update]
#[candid_method(update)]
fn send_dm(id: u64, sender_id: u64, receiver_id: u64, content: String) {
    let dm = DirectMessage {
        id,
        sender_id,
        receiver_id,
        content,
        timestamp: now_seconds(),
    };
    DMS.with(|dms| {
        dms.borrow_mut().insert(id, dm);
    });
}

#[query]
#[candid_method(query)]
fn get_dms(user_id: u64) -> Vec<DirectMessage> {
    DMS.with(|dms| {
        dms.borrow()
            .iter()
            .filter(|(_, dm)| dm.receiver_id == user_id || dm.sender_id == user_id)
            .map(|(_, dm)| dm.clone())
            .collect()
    })
}

// ---------- EXPORT CANDID ---------- //
ic_cdk::export_candid!();
