mod storage;
mod utils;
mod models;


use ic_cdk_macros::{query, update};
use models::User;

fn create_user(username: String, bio: String) -> u64 {
    let id = storage::next_user_id();
    let user = User {
        id,
        username,
        bio,
        joined_at: utils::now_nanos(),
    };
    storage::users_put(id, &user);
    id
}

fn get_user(id: u64) -> Option<User> {
    storage::users_get::<User>(id)
}