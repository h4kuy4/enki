use serde::Serialize;

use crate::model;

#[derive(Debug, Serialize)]
pub struct Category {
    pub id: i32,
    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_count: Option<i64>,
}

impl Category {
    pub fn serialize(model: model::Category) -> Self {
        match model {
            model::Category::Full {
                id,
                name,
                post_count,
            } => Self {
                id,
                name,
                post_count,
            },
            _ => {
                log::error!("Wrong model!");
                panic!()
            }
        }
    }
}
