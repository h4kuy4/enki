use sea_orm::ActiveModelTrait;
use sea_orm::ActiveValue::Set;
use sea_orm::DatabaseConnection;
use sea_orm::EntityTrait;
use sea_orm::PaginatorTrait;

use crate::deserializer;
use crate::entity;
use crate::entity::TagModel;
use crate::entity::TagPostCountModel;
use crate::Error;
use crate::ErrorType;
use crate::Result;

#[derive(Debug)]
pub enum Tag {
    Full {
        id: i32,
        name: String,
        post_count: Option<i64>,
    },
    IdOnly {
        id: i32,
    },
    NameOnly {
        name: String,
    },
}

impl Tag {
    pub async fn get_by_id(conn: &DatabaseConnection, id: i32) -> Result<Tag> {
        let tag = entity::TagPostCount::find_by_id(id)
            .one(conn)
            .await?
            .ok_or(Error::new("Tag", "Tag not found", ErrorType::TagNotFound))?;

        Ok(Tag::from(tag))
    }

    pub async fn get_list(
        conn: &DatabaseConnection,
        page_size: Option<u64>,
        page: Option<u64>,
    ) -> Result<(Vec<Tag>, u64)> {
        let page_size = match page_size {
            Some(page_size) => page_size,
            None => 10,
        };

        let page = match page {
            Some(page) => page,
            None => 1,
        };

        let pages = entity::TagPostCount::find().paginate(conn, page_size);
        let page_num = pages.num_pages().await?;
        let tags = pages.fetch_page(page - 1).await?;

        let tags = tags.into_iter().map(|tag| Tag::from(tag)).collect();

        Ok((tags, page_num))
    }

    pub async fn insert(conn: &DatabaseConnection, model: Tag) -> Result<i32> {
        let name = match model {
            Tag::NameOnly { name } => name,
            _ => {
                log::error!("Tag Model: Wrong model!");
                panic!();
            }
        };

        let model = entity::tag::ActiveModel {
            name: Set(name),
            ..Default::default()
        };

        let model: entity::tag::Model = model.insert(conn).await?;

        Ok(model.id)
    }

    pub async fn update(conn: &DatabaseConnection, id: i32, model: Tag) -> Result<i32> {
        let name = match model {
            Tag::NameOnly { name } => name,
            _ => {
                log::error!("Tag Model: Wrong model!");
                panic!();
            }
        };

        let mut act_model: entity::tag::ActiveModel = entity::Tag::find_by_id(id)
            .one(conn)
            .await?
            .ok_or(Error::new("Tag", "Tag not found", ErrorType::TagNotFound))?
            .into();

        act_model.name = Set(name);

        let model = act_model.update(conn).await?;

        Ok(model.id)
    }

    pub async fn delete(conn: &DatabaseConnection, id: i32) -> Result<()> {
        entity::Tag::find_by_id(id)
            .one(conn)
            .await?
            .ok_or(Error::new("Tag", "Tag not found", ErrorType::TagNotFound))?;

        entity::Tag::delete_by_id(id).exec(conn).await?;

        Ok(())
    }

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
            Self::IdOnly { id } => id.to_owned(),
            Self::NameOnly { name: _ } => {
                log::error!("ID feild not found.");
                panic!()
            }
        }
    }
}

impl From<TagModel> for Tag {
    fn from(model: TagModel) -> Self {
        Self::Full {
            id: model.id,
            name: model.name,
            post_count: None,
        }
    }
}

impl From<i32> for Tag {
    fn from(id: i32) -> Self {
        Self::from_id(id)
    }
}

impl From<TagPostCountModel> for Tag {
    fn from(model: TagPostCountModel) -> Self {
        Self::Full {
            id: model.id,
            name: model.name,
            post_count: Some(model.count),
        }
    }
}

impl From<deserializer::Tag> for Tag {
    fn from(model: deserializer::Tag) -> Self {
        Self::NameOnly { name: model.name }
    }
}
