use serde::Deserialize;

#[derive(Deserialize)]
pub struct Post {
    pub title: String,
    pub description: String,
    pub content: String,
    pub category: i32,
    pub publish: bool,
}
