use candid::{CandidType, Deserialize};
use ic_cdk::api::time;

use crate::storage::{
    create_user, users_get, credentials_get, credentials_put,
    all_user_ids, find_user_by_username,
};
use crate::models::User;


#[derive(CandidType, Deserialize, Clone)]
pub struct Credential {
    pub user_id: u64,
    pub username: String,
    pub email: String,
    pub password: String,
    pub reset_code: Option<String>,
    pub reset_expires_at: Option<u64>,
}

#[derive(CandidType, Deserialize, Clone)]
pub struct AuthResult {
    pub success: bool,
    pub message: Option<String>,
}

fn ok() -> AuthResult { AuthResult { success: true, message: None } }
fn err(msg: &str) -> AuthResult { AuthResult { success: false, message: Some(msg.into()) } }

pub fn signup(username: String, email: String, password: String, bio: Option<String>) -> Result<User, AuthResult> {

    if find_user_by_username(&username).is_some() {
        return Err(err("Username already exists"));
    }

    for uid in all_user_ids() {
        if let Some(c) = credentials_get::<Credential>(uid) {
            if c.email == email {
                return Err(err("Email already in use"));
            }
        }
    }

    let user = create_user(username.clone(), bio);

    let cred = Credential {
        user_id: user.id,
        username,
        email,
        password,
        reset_code: None,
        reset_expires_at: None,
    };
    credentials_put(user.id, &cred);

    Ok(user)
}

pub fn login(username: String, password: String) -> AuthResult {
    let Some(user) = find_user_by_username(&username) else {
        return err("User not found");
    };
    if let Some(cred) = credentials_get::<Credential>(user.id) {
        if cred.password == password {
            ok()
        } else {
            err("Invalid credentials")
        }
    } else {
        err("Credentials not found")
    }
}

pub fn request_password_reset(email: String) -> Result<String, AuthResult> {
    let mut target: Option<Credential> = None;
    for uid in all_user_ids() {
        if let Some(c) = credentials_get::<Credential>(uid) {
            if c.email == email {
                target = Some(c);
                break;
            }
        }
    }
    let Some(mut cred) = target else { return Err(err("Email not found")); };
    let code = format!("R{}", time());
    let now_sec = time() / 1_000_000_000;
    cred.reset_code = Some(code.clone());
    cred.reset_expires_at = Some(now_sec + 15 * 60);
    credentials_put(cred.user_id, &cred);
    Ok(code)
}

pub fn reset_password(email: String, code: String, new_password: String) -> AuthResult {
    let mut target: Option<Credential> = None;
    for uid in all_user_ids() {
        if let Some(c) = credentials_get::<Credential>(uid) {
            if c.email == email {
                target = Some(c);
                break;
            }
        }
    }
    let Some(mut cred) = target else { return err("Email not found"); };

    let now_sec = time() / 1_000_000_000;
    match (&cred.reset_code, cred.reset_expires_at) {
        (Some(stored), Some(exp)) if *stored == code && now_sec <= exp => {
            cred.password = new_password;
            cred.reset_code = None;
            cred.reset_expires_at = None;
            credentials_put(cred.user_id, &cred);
            ok()
        }
        (Some(_), Some(_)) => err("Invalid or expired reset code"),
        _ => err("No reset requested"),
    }
}