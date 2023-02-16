use serde::Serialize;

use crate::model;

#[derive(Debug, Serialize)]
pub struct Category {
    pub id: i32,
    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_count: Option<i32>,
}

impl Category {
    pub fn serialize(model: model::Category) -> Self {
        match model {
            model::Category::IdOnly { id: _ } => {
                log::error!("Category Serializer: Wrong model!");
                panic!()
            }
            model::Category::WithoutCount { id, name } => Self {
                id,
                name,
                post_count: None,
            },
            model::Category::Full {
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
