use crate::entity::CategoryModel;

#[derive(Debug)]
pub enum Category {
    Full {
        id: i32,
        name: String,
        post_count: i32,
    },
    WithoutCount {
        id: i32,
        name: String,
    },
    IdOnly {
        id: i32,
    },
}

impl Category {
    pub fn from_id(id: i32) -> Self {
        Self::IdOnly { id }
    }

    pub fn get_id(&self) -> i32 {
        match &self {
            Self::Full {
                id,
                name: _,
                post_count: _,
            } => id.to_owned(),
            Self::WithoutCount { id, name: _ } => id.to_owned(),
            Self::IdOnly { id } => id.to_owned(),
        }
    }
}

impl From<CategoryModel> for Category {
    fn from(model: CategoryModel) -> Self {
        Self::WithoutCount {
            id: model.id,
            name: model.name,
        }
    }
}

impl From<i32> for Category {
    fn from(id: i32) -> Self {
        Self::from_id(id)
    }
}
