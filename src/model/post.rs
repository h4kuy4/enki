use chrono::NaiveDateTime;

use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait, LoaderTrait,
    ModelTrait, PaginatorTrait, QueryFilter,
};

use super::category::Category;

use crate::entity::{CategoryModel, PostModel};
use crate::Error;
use crate::ErrorType;
use crate::Result;
use crate::{deserializer, entity};

#[derive(Debug)]
pub struct Post {
    pub id: Option<i32>,
    pub title: String,
    pub description: String,
    pub content: String,
    pub category: Category,
    pub public_at: Option<NaiveDateTime>,
}

impl Post {
    pub async fn get_by_id(conn: &DatabaseConnection, id: i32) -> Result<Post> {
        let post = entity::Post::find_by_id(id);

        let post = post.one(conn).await?.ok_or(Error::new(
            "Post",
            "Post not found",
            ErrorType::PostNotFound,
        ))?;

        let category: Option<CategoryModel> = post.find_related(entity::Category).one(conn).await?;

        Ok(Post::from((post, category)))
    }

    pub async fn get_list(
        conn: &DatabaseConnection,
        page_size: Option<u64>,
        page: Option<u64>,
    ) -> Result<(Vec<Post>, u64)> {
        let page_size = match page_size {
            Some(page_size) => page_size,
            None => 10,
        };

        let page = match page {
            Some(page) => page,
            None => 1,
        };

        let post = entity::Post::find();

        let pages = post.paginate(conn, page_size);

        let page_num = pages.num_pages().await?;
        let posts = pages.fetch_page(page - 1).await?;

        let categories: Vec<Option<CategoryModel>> = posts.load_one(entity::Category, conn).await?;

        let posts: Vec<Post> = posts
            .into_iter()
            .zip(categories)
            .map(|(a, b)| Post::from((a, b)))
            .collect();

        Ok((posts, page_num))
    }

    pub async fn get_list_by_category(
        conn: &DatabaseConnection,
        category_id: i32,
        page_size: Option<u64>,
        page: Option<u64>,
    ) -> Result<(Vec<Post>, u64)> {
        let page_size = match page_size {
            Some(page_size) => page_size,
            None => 10,
        };

        let page = match page {
            Some(page) => page,
            None => 1,
        };

        let post = entity::Post::find().filter(entity::post::Column::CategoryId.eq(category_id));

        let pages = post.paginate(conn, page_size);

        let page_num = pages.num_pages().await?;
        let posts = pages.fetch_page(page - 1).await?;

        let categories: Vec<Option<CategoryModel>> = posts.load_one(entity::Category, conn).await?;

        let posts: Vec<Post> = posts
            .into_iter()
            .zip(categories)
            .map(|(a, b)| Post::from((a, b)))
            .collect();

        Ok((posts, page_num))
    }

    pub async fn insert(conn: &DatabaseConnection, model: Post) -> Result<i32> {
        let category_id = model.category.id.ok_or(Error::new(
            "Post",
            "Category ID required.",
            ErrorType::RequestError,
        ))?;

        let _category = entity::Category::find_by_id(category_id)
            .one(conn)
            .await?
            .ok_or(Error::new(
                "Post",
                "Category not found",
                ErrorType::CategoryNotFound,
            ))?;

        let model = entity::post::ActiveModel {
            title: Set(model.title),
            description: Set(model.description),
            content: Set(model.content),
            category_id: Set(category_id),
            created_at: Set(chrono::Local::now().naive_local()),
            public_at: Set(model.public_at),
            ..Default::default()
        };

        let model: entity::post::Model = model.insert(conn).await?;

        Ok(model.id)
    }

    pub async fn update(conn: &DatabaseConnection, id: i32, model: Post) -> Result<i32> {
        let category_id = model.category.id.ok_or(Error::new(
            "Post",
            "Category ID required.",
            ErrorType::RequestError,
        ))?;

        let _category = entity::Category::find_by_id(category_id)
            .one(conn)
            .await?
            .ok_or(Error::new(
                "Post",
                "Category not found",
                ErrorType::CategoryNotFound,
            ))?;

        let mut act_model: entity::post::ActiveModel = entity::Post::find_by_id(id)
            .one(conn)
            .await?
            .ok_or(Error::new(
                "Post",
                "Post Not Found",
                ErrorType::PostNotFound,
            ))?
            .into();

        act_model.title = Set(model.title);
        act_model.description = Set(model.description);
        act_model.content = Set(model.content);
        act_model.category_id = Set(category_id);
        act_model.public_at = Set(model.public_at);

        let model: entity::post::Model = act_model.update(conn).await?;

        Ok(model.id)
    }

    pub async fn delete(conn: &DatabaseConnection, id: i32) -> Result<()> {
        entity::Post::find_by_id(id)
            .one(conn)
            .await?
            .ok_or(Error::new(
                "Post",
                "Post Not Found",
                ErrorType::PostNotFound,
            ))?;

        entity::Post::delete_by_id(id).exec(conn).await?;

        Ok(())
    }
}

impl From<(PostModel, Option<CategoryModel>)> for Post {
    fn from(item: (PostModel, Option<CategoryModel>)) -> Post {
        let (post, category) = item;

        let category = match category {
            Some(category) => Category::from(category),
            None => {
                panic!("Category not found for post {}", post.id);
            }
        };

        Self {
            id: Some(post.id),
            title: post.title,
            description: post.description,
            content: post.content,
            category,
            public_at: post.public_at,
        }
    }
}

impl From<deserializer::Post> for Post {
    fn from(post: deserializer::Post) -> Self {
        Self {
            id: None,
            title: post.title,
            description: post.description,
            content: post.content,
            category: Category::from(post.category),
            public_at: if post.publish {
                Some(chrono::Local::now().naive_local())
            } else {
                None
            },
        }
    }
}
