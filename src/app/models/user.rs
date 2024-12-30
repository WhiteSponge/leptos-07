use reactive_stores::{Field, Patch, Store};
use serde::{Deserialize, Serialize};

#[derive(Debug, Store, Serialize, Deserialize)]
pub struct UserStore {
    pub user: User,
}

#[derive(Debug, Store, Patch, Serialize, Deserialize, Clone)]
pub struct User {
    pub name: String,
    pub email: String,
    pub is_authenticated: bool,
}

impl User {
    pub fn new(name: impl Into<String>, email: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            email: email.into(),
            is_authenticated: false,
        }
    }
}
