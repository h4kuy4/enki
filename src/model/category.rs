use sea_orm::ActiveModelTrait;
use sea_orm::ActiveValue::Set;
use sea_orm::DatabaseConnection;
use sea_orm::EntityTrait;
use sea_orm::PaginatorTrait;

use crate::deserializer;
use crate::entity;
use crate::entity::CatePostCountModel;
use crate::entity::CategoryModel;
use crate::Error;
use crate::ErrorType;
use crate::Result;

#[derive(Debug)]
pub enum Category {
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

impl Category {
    pub async fn get_by_id(conn: &DatabaseConnection, id: i32) -> Result<Category> {
        let category = entity::CatePostCount::find_by_id(id)
            .one(conn)
            .await?
            .ok_or(Error::new(
                "Category",
                "Category not found",
                ErrorType::TagNotFound,
            ))?;

        Ok(Category::from(category))
    }

    pub async fn get_list(
        conn: &DatabaseConnection,
        page_size: Option<u64>,
        page: Option<u64>,
    ) -> Result<(Vec<Category>, u64)> {
        let page_size = match page_size {
            Some(page_size) => page_size,
            None => 10,
        };

        let page = match page {
            Some(page) => page,
            None => 1,
        };

        let pages = entity::CatePostCount::find().paginate(conn, page_size);
        let page_num = pages.num_pages().await?;
        let tags = pages.fetch_page(page - 1).await?;

        let tags = tags
            .into_iter()
            .map(|category| Category::from(category))
            .collect();

        Ok((tags, page_num))
    }

    pub async fn insert(conn: &DatabaseConnection, model: Category) -> Result<i32> {
        let name = match model {
            Category::NameOnly { name } => name,
            _ => {
                log::error!("Category Model: Wrong model!");
                panic!();
            }
        };

        let model = entity::category::ActiveModel {
            name: Set(name),
            ..Default::default()
        };

        let model: entity::category::Model = model.insert(conn).await?;

        Ok(model.id)
    }

    pub async fn update(conn: &DatabaseConnection, id: i32, model: Category) -> Result<i32> {
        let name = match model {
            Category::NameOnly { name } => name,
            _ => {
                log::error!("Category Model: Wrong model!");
                panic!();
            }
        };

        let mut act_model: entity::category::ActiveModel = entity::Category::find_by_id(id)
            .one(conn)
            .await?
            .ok_or(Error::new(
                "Category",
                "Category not found",
                ErrorType::CategoryNotFound,
            ))?
            .into();

        act_model.name = Set(name);

        let model = act_model.update(conn).await?;

        Ok(model.id)
    }

    pub async fn delete(conn: &DatabaseConnection, id: i32) -> Result<()> {
        entity::Category::find_by_id(id)
            .one(conn)
            .await?
            .ok_or(Error::new(
                "Category",
                "Category not found",
                ErrorType::CategoryNotFound,
            ))?;

        entity::Category::delete_by_id(id).exec(conn).await?;

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

impl From<CategoryModel> for Category {
    fn from(model: CategoryModel) -> Self {
        Self::Full {
            id: model.id,
            name: model.name,
            post_count: None,
        }
    }
}

impl From<i32> for Category {
    fn from(id: i32) -> Self {
        Self::from_id(id)
    }
}

impl From<CatePostCountModel> for Category {
    fn from(model: CatePostCountModel) -> Self {
        Self::Full {
            id: model.id,
            name: model.name,
            post_count: Some(model.count),
        }
    }
}

impl From<deserializer::Category> for Category {
    fn from(model: deserializer::Category) -> Self {
        Self::NameOnly { name: model.name }
    }
}
