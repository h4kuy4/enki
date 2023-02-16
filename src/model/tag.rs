use crate::entity::TagModel;

#[derive(Debug)]
pub enum Tag {
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

impl Tag {
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

impl From<TagModel> for Tag {
    fn from(model: TagModel) -> Self {
        Self::WithoutCount {
            id: model.id,
            name: model.name,
        }
    }
}

impl From<i32> for Tag {
    fn from(id: i32) -> Self {
        Self::from_id(id)
    }
}
