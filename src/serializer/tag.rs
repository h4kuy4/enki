use serde::Serialize;

use crate::model;

#[derive(Debug, Serialize)]
pub struct Tag {
    pub id: i32,
    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_count: Option<i32>,
}

impl Tag {
    pub fn serialize(model: model::Tag) -> Self {
        match model {
            model::Tag::IdOnly { id: _ } => {
                log::error!("Tag Serializer: Wrong model!");
                panic!()
            }
            model::Tag::WithoutCount { id, name } => Self {
                id,
                name,
                post_count: None,
            },
            model::Tag::Full {
                id,
                name,
                post_count,
            } => Self {
                id,
                name,
                post_count: Some(post_count),
            },
        }
    }
}
