use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Account {
    pub user_name: String,
    pub password: String,
}
