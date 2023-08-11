use crate::model;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Friend {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub avatar_url: String,
    pub url: String,
}

impl Friend {
    pub fn serialize(model: model::Friend) -> Self {
        Self {
            id: model.id,
            name: model.name,
            description: model.description,
            avatar_url: model.avatar_url,
            url: model.url,
        }
    }
}
