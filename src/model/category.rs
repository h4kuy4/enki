use sea_orm::ActiveModelTrait;
use sea_orm::ActiveValue::Set;
use sea_orm::DatabaseConnection;
use sea_orm::EntityTrait;
use sea_orm::PaginatorTrait;

use crate::deserializer;
use crate::entity;
use crate::entity::CategoryModel;
use crate::Error;
use crate::ErrorType;
use crate::Result;

#[derive(Debug)]
pub struct Category {
    pub id: Option<i32>,
    pub name: Option<String>,
}

impl Category {
    pub async fn get_by_id(conn: &DatabaseConnection, id: i32) -> Result<Category> {
        let category = entity::Category::find_by_id(id)
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

        let pages = entity::Category::find().paginate(conn, page_size);
        let page_num = pages.num_pages().await?;
        let tags = pages.fetch_page(page - 1).await?;

        let tags = tags
            .into_iter()
            .map(|category| Category::from(category))
            .collect();

        Ok((tags, page_num))
    }

    pub async fn insert(conn: &DatabaseConnection, model: Category) -> Result<i32> {
        let name = model.name.ok_or(Error::new(
            "Category",
            "Category name is required",
            ErrorType::RequestError,
        ))?;

        let model = entity::category::ActiveModel {
            name: Set(name),
            ..Default::default()
        };

        let model: entity::category::Model = model.insert(conn).await?;

        Ok(model.id)
    }

    pub async fn update(conn: &DatabaseConnection, id: i32, model: Category) -> Result<i32> {
        let name = model.name.ok_or(Error::new(
            "Category",
            "Category name is required",
            ErrorType::RequestError,
        ))?;

        let mut active_model: entity::category::ActiveModel = entity::Category::find_by_id(id)
            .one(conn)
            .await?
            .ok_or(Error::new(
                "Category",
                "Category not found",
                ErrorType::CategoryNotFound,
            ))?
            .into();

        active_model.name = Set(name);

        let model = active_model.update(conn).await?;

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
}

impl From<CategoryModel> for Category {
    fn from(model: CategoryModel) -> Self {
        Self {
            id: Some(model.id),
            name: Some(model.name),
        }
    }
}

impl From<i32> for Category {
    fn from(id: i32) -> Self {
        Self {
            id: Some(id),
            name: None,
        }
    }
}

impl From<deserializer::Category> for Category {
    fn from(model: deserializer::Category) -> Self {
        Self {
            id: None,
            name: Some(model.name),
        }
    }
}
