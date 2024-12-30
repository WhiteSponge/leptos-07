use crate::app::models::User;
use cfg_if::cfg_if;
use leptos::prelude::*;

#[server(GetAllUsers, "/api")]
pub async fn get_all_users() -> Result<Vec<User>, ServerFnError> {
    let users = retrieve_all_users().await;
    Ok(users)
}

cfg_if! {
    if #[cfg(feature = "ssr")] {

        pub async fn retrieve_all_users() -> Vec<User> {
            vec![
                User::new("tom","tom@gmail.com"),
                User::new("Christy","christy@gmail.com")
            ]
        }
    }
}
