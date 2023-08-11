use serde::Serialize;

use crate::model;

#[derive(Debug, Serialize)]
pub struct Category {
    pub id: i32,
    pub name: String,
}

impl Category {
    pub fn serialize(model: model::Category) -> Self {
        let id = match model.id {
            Some(id) => id,
            None => {
                panic!("Category model id is None");
            }
        };

        let name = match model.name {
            Some(name) => name,
            None => {
                panic!("Category model name is None");
            }
        };

        Self { id, name }
    }
}
