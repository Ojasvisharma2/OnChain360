use candid::{CandidType, Decode, Encode, Deserialize};
use ic_stable_structures::{
    memory_manager::{MemoryId, MemoryManager, VirtualMemory},
    DefaultMemoryImpl, StableBTreeMap, StableCell,
};
use std::cell::RefCell;

use crate::models::{User, Post, Comment, Notification, DirectMessage};

type Mem = DefaultMemoryImpl;
type VMem = VirtualMemory<Mem>;
type Bytes = Vec<u8>;
type MapBytes = StableBTreeMap<u64, Bytes, VMem>;

thread_local! {
    pub static MEMORY_MANAGER: RefCell<MemoryManager<Mem>> =
        RefCell::new(MemoryManager::init(Mem::default()));

    // Entity Maps
    pub static USERS: RefCell<MapBytes> = RefCell::new(
        StableBTreeMap::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))))
    );
    pub static POSTS: RefCell<MapBytes> = RefCell::new(
        StableBTreeMap::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1))))
    );
    pub static COMMENTS: RefCell<MapBytes> = RefCell::new(
        StableBTreeMap::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(2))))
    );
    pub static NOTIFICATIONS: RefCell<MapBytes> = RefCell::new(
        StableBTreeMap::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(3))))
    );
    pub static DMS: RefCell<MapBytes> = RefCell::new(
        StableBTreeMap::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(4))))
    );

    // Credentials (auth-only)
    pub static CREDENTIALS: RefCell<MapBytes> = RefCell::new(
        StableBTreeMap::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(5))))
    );

    // ID Counters
    pub static NEXT_USER_ID: RefCell<StableCell<u64, VMem>> = RefCell::new(
        StableCell::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(10))), 0u64)
    );
    pub static NEXT_POST_ID: RefCell<StableCell<u64, VMem>> = RefCell::new(
        StableCell::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(11))), 0u64)
    );
    pub static NEXT_COMMENT_ID: RefCell<StableCell<u64, VMem>> = RefCell::new(
        StableCell::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(12))), 0u64)
    );
    pub static NEXT_NOTIFICATION_ID: RefCell<StableCell<u64, VMem>> = RefCell::new(
        StableCell::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(13))), 0u64)
    );
    pub static NEXT_DM_ID: RefCell<StableCell<u64, VMem>> = RefCell::new(
        StableCell::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(14))), 0u64)
    );
}

// ID helpers
macro_rules! next_id {
    ($cell:expr) => {{
        $cell.with(|c| {
            let cur = *c.borrow().get();
            c.borrow_mut().set(cur + 1);
            cur + 1
        })
    }};
}

pub fn next_user_id() -> u64 { next_id!(NEXT_USER_ID) }
pub fn next_post_id() -> u64 { next_id!(NEXT_POST_ID) }
pub fn next_comment_id() -> u64 { next_id!(NEXT_COMMENT_ID) }
pub fn next_notification_id() -> u64 { next_id!(NEXT_NOTIFICATION_ID) }
pub fn next_dm_id() -> u64 { next_id!(NEXT_DM_ID) }

// Generic CRUD
macro_rules! gen_crud {
    ($get_fn:ident, $put_fn:ident, $remove_fn:ident, $map:ident) => {
        pub fn $get_fn<T: CandidType + for<'de> Deserialize<'de>>(id: u64) -> Option<T> {
            $map.with(|m| {
                m.borrow()
                    .get(&id)
                    .and_then(|b| Decode!(&b, T).ok())
            })
        }
        pub fn $put_fn<T: CandidType>(id: u64, value: &T) {
            if let Ok(bytes) = Encode!(value) {
                $map.with(|m| { m.borrow_mut().insert(id, bytes); });
            }
        }
        pub fn $remove_fn(id: u64) -> Option<Bytes> {
            $map.with(|m| m.borrow_mut().remove(&id))
        }
    }
}

gen_crud!(users_get, users_put, users_remove, USERS);
gen_crud!(posts_get, posts_put, posts_remove, POSTS);
gen_crud!(comments_get, comments_put, comments_remove, COMMENTS);
gen_crud!(notifications_get, notifications_put, notifications_remove, NOTIFICATIONS);
gen_crud!(dms_get, dms_put, dms_remove, DMS);
gen_crud!(credentials_get, credentials_put, credentials_remove, CREDENTIALS);

// Create & store user with auto-id
pub fn create_user(username: String, bio: Option<String>) -> User {
    let id = next_user_id();
    let user = User {
        id,
        username,
        bio,
        followers: Vec::new(),
        following: Vec::new(),
    };
    users_put(id, &user);
    user
}

// Follow/unfollow
pub fn follow_user(follower_id: u64, followee_id: u64) -> bool {
    let mut changed = false;
    if let Some(mut follower) = users_get::<User>(follower_id) {
        if let Some(mut followee) = users_get::<User>(followee_id) {
            if !follower.following.contains(&followee_id) {
                follower.following.push(followee_id);
                users_put(follower_id, &follower);
                changed = true;
            }
            if !followee.followers.contains(&follower_id) {
                followee.followers.push(follower_id);
                users_put(followee_id, &followee);
                changed = true;
            }
        }
    }
    changed
}

pub fn unfollow_user(follower_id: u64, followee_id: u64) -> bool {
    let mut changed = false;
    if let Some(mut follower) = users_get::<User>(follower_id) {
        if let Some(mut followee) = users_get::<User>(followee_id) {
            let before = follower.following.len();
            follower.following.retain(|&id| id != followee_id);
            if follower.following.len() != before {
                users_put(follower_id, &follower);
                changed = true;
            }
            let before2 = followee.followers.len();
            followee.followers.retain(|&id| id != follower_id);
            if followee.followers.len() != before2 {
                users_put(followee_id, &followee);
                changed = true;
            }
        }
    }
    changed
}

// Create post
pub fn create_post(author_id: u64, content: String, timestamp: u64) -> u64 {
    let id = next_post_id();
    let post = Post {
        id,
        author_id,
        content,
        likes: Vec::new(),
        comments: Vec::new(),
        timestamp,
    };
    posts_put(id, &post);
    id
}

// Like/unlike
pub fn like_post(user_id: u64, post_id: u64) -> bool {
    if let Some(mut post) = posts_get::<Post>(post_id) {
        if !post.likes.contains(&user_id) {
            post.likes.push(user_id);
            posts_put(post_id, &post);
            return true;
        }
    }
    false
}
pub fn unlike_post(user_id: u64, post_id: u64) -> bool {
    if let Some(mut post) = posts_get::<Post>(post_id) {
        let before = post.likes.len();
        post.likes.retain(|&u| u != user_id);
        if post.likes.len() != before {
            posts_put(post_id, &post);
            return true;
        }
    }
    false
}

// Add comment + attach to post
pub fn add_comment(post_id: u64, author_id: u64, content: String, timestamp: u64) -> u64 {
    let id = next_comment_id();
    let comment = Comment { id, post_id, author_id, content, timestamp };
    comments_put(id, &comment);
    if let Some(mut post) = posts_get::<Post>(post_id) {
        post.comments.push(id);
        posts_put(post_id, &post);
    }
    id
}

// Notifications
pub fn add_notification(user_id: u64, message: String, timestamp: u64) -> u64 {
    let id = next_notification_id();
    let notif = Notification { id, user_id, message, read: false, timestamp };
    notifications_put(id, &notif);
    id
}

pub fn list_notifications(user_id: u64) -> Vec<Notification> {
    let max = NEXT_NOTIFICATION_ID.with(|c| *c.borrow().get());
    (0..max)
        .filter_map(|id| notifications_get::<Notification>(id))
        .filter(|n| n.user_id == user_id)
        .collect()
}

// DMs
pub fn send_dm(sender_id: u64, receiver_id: u64, content: String, timestamp: u64) -> u64 {
    let id = next_dm_id();
    let dm = DirectMessage { id, sender_id, receiver_id, content, timestamp };
    dms_put(id, &dm);
    id
}
pub fn list_dms(user_id: u64) -> Vec<DirectMessage> {
    let max = NEXT_DM_ID.with(|c| *c.borrow().get());
    (0..max)
        .filter_map(|id| dms_get::<DirectMessage>(id))
        .filter(|dm| dm.receiver_id == user_id || dm.sender_id == user_id)
        .collect()
}

// Utility scans
pub fn find_user_by_username(username: &str) -> Option<User> {
    let max = NEXT_USER_ID.with(|c| *c.borrow().get());
    (0..max)
        .filter_map(|id| users_get::<User>(id))
        .find(|u| u.username == username)
}

pub fn all_user_ids() -> Vec<u64> {
    let max = NEXT_USER_ID.with(|c| *c.borrow().get());
    (0..max).collect()
}