use chrono::NaiveDateTime;

use sea_orm::{
    sea_query::Query, ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection,
    EntityTrait, LoaderTrait, ModelTrait, PaginatorTrait, QueryFilter, Select,
};

use super::category::Category;
use super::tag::Tag;

use crate::entity::{CategoryModel, PostModel, TagModel};
use crate::Error;
use crate::ErrorType;
use crate::Result;
use crate::{deserializer, entity};

#[derive(Debug)]
pub enum Post {
    WithID(PostWithID),
    WithoutID(PostWithoutID),
}

#[derive(Debug)]
pub struct PostWithID {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub content: String,
    pub category: Option<Category>,
    pub tags: Vec<Tag>,
    pub public_at: Option<NaiveDateTime>,
}

#[derive(Debug)]
pub struct PostWithoutID {
    pub title: String,
    pub description: String,
    pub content: String,
    pub category: Option<Category>,
    pub tags: Vec<Tag>,
    pub public_at: Option<NaiveDateTime>,
}

#[derive(Debug)]
pub enum PostFor {
    Frontend,
    Backend,
}

#[derive(Debug)]
pub enum ListType {
    Full,
    ForCategory(i32),
    ForTag(i32),
}

impl Post {
    pub async fn get_by_id(conn: &DatabaseConnection, post_for: PostFor, id: i32) -> Result<Post> {
        let post = entity::Post::find_by_id(id);

        let post = PostFor::filter(post, post_for);

        let post = post.one(conn).await?.ok_or(Error::new(
            "Post",
            "Post Not Found",
            ErrorType::PostNotFound,
        ))?;

        let category: Option<CategoryModel> = post.find_related(entity::Category).one(conn).await?;
        let tags: Vec<TagModel> = post.find_related(entity::Tag).all(conn).await?;

        Ok(Post::from((post, category, tags)))
    }

    pub async fn get_list(
        conn: &DatabaseConnection,
        post_for: PostFor,
        list_type: ListType,
        page_size: Option<u64>,
        page: Option<u64>,
    ) -> Result<(Vec<Post>, u64)> {
        let page_size = match page_size {
            Some(page_size) => page_size,
            None => 10,
        };

        let page = match page {
            Some(page) => page,
            None => 0,
        };

        let post = entity::Post::find();

        let post = PostFor::filter(post, post_for);
        let post = ListType::filter(post, list_type);

        let pages = post.paginate(conn, page_size);

        let page_num = pages.num_pages().await?;
        let posts = pages.fetch_page(page - 1).await?;
        let categories: Vec<Option<CategoryModel>> = posts.load_one(entity::Category, conn).await?;
        let tags: Vec<Vec<TagModel>> = posts
            .load_many_to_many(entity::Tag, entity::PostTag, conn)
            .await?;

        let posts: Vec<Post> = posts
            .into_iter()
            .zip(categories)
            .zip(tags)
            .map(|((a, b), c)| Post::from((a, b, c)))
            .collect();

        Ok((posts, page_num))
    }

    pub async fn insert(conn: &DatabaseConnection, model: Post) -> Result<i32> {
        let model = match model {
            Post::WithID(_) => {
                log::error!("Post Model: Wrong model!");
                panic!()
            }
            Post::WithoutID(model) => model,
        };

        let tags = model.tags;

        if let Some(category) = model.category.as_ref() {
            entity::Category::find_by_id(category.get_id())
                .one(conn)
                .await?
                .ok_or(Error::new(
                    "Category",
                    "Category not found.",
                    ErrorType::TagNotFound,
                ))?;
        }

        for tag in tags.iter() {
            entity::Tag::find_by_id(tag.get_id())
                .one(conn)
                .await?
                .ok_or(Error::new("Tag", "Tag not found.", ErrorType::TagNotFound))?;
        }

        let model = entity::post::ActiveModel {
            title: Set(model.title),
            description: Set(model.description),
            content: Set(model.content),
            category_id: Set(model.category.map(|category| category.get_id())),
            created_at: Set(chrono::Local::now().naive_local()),
            public_at: Set(model.public_at),
            ..Default::default()
        };

        let model: entity::post::Model = model.insert(conn).await?;

        let tags: Vec<entity::post_tag::ActiveModel> = tags
            .into_iter()
            .map(|tag| entity::post_tag::ActiveModel {
                post_id: Set(model.id),
                tag_id: Set(tag.get_id()),
                ..Default::default()
            })
            .collect();
        entity::PostTag::insert_many(tags).exec(conn).await?;

        Ok(model.id)
    }

    pub async fn update(conn: &DatabaseConnection, id: i32, model: Post) -> Result<i32> {
        let model = match model {
            Post::WithID(_) => {
                log::error!("Post Model: Wrong model!");
                panic!()
            }
            Post::WithoutID(model) => model,
        };
        let tags = model.tags;

        if let Some(category) = model.category.as_ref() {
            entity::Category::find_by_id(category.get_id())
                .one(conn)
                .await?
                .ok_or(Error::new(
                    "Category",
                    "Category not found.",
                    ErrorType::TagNotFound,
                ))?;
        }

        for tag in tags.iter() {
            entity::Tag::find_by_id(tag.get_id())
                .one(conn)
                .await?
                .ok_or(Error::new("Tag", "Tag not found.", ErrorType::TagNotFound))?;
        }

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
        act_model.category_id = Set(model.category.map(|category| category.get_id()));
        act_model.public_at = Set(model.public_at);

        let model: entity::post::Model = act_model.update(conn).await?;

        entity::PostTag::delete_many()
            .filter(entity::post_tag::Column::PostId.eq(model.id))
            .exec(conn)
            .await?;

        let tags: Vec<entity::post_tag::ActiveModel> = tags
            .into_iter()
            .map(|tag| entity::post_tag::ActiveModel {
                post_id: Set(model.id),
                tag_id: Set(tag.get_id()),
                ..Default::default()
            })
            .collect();
        entity::PostTag::insert_many(tags).exec(conn).await?;

        Ok(model.id)
    }

    pub async fn delete(conn: &DatabaseConnection, id: i32) -> Result<()> {
        entity::PostTag::delete_many()
            .filter(entity::post_tag::Column::PostId.eq(id))
            .exec(conn)
            .await?;

        entity::Post::delete_by_id(id).exec(conn).await?;

        Ok(())
    }
}

impl PostFor {
    pub fn filter(post: Select<entity::Post>, post_for: PostFor) -> Select<entity::Post> {
        match post_for {
            PostFor::Frontend => post.filter(entity::post::Column::PublicAt.is_not_null()),
            PostFor::Backend => post,
        }
    }
}

impl ListType {
    pub fn filter(post: Select<entity::Post>, list_type: ListType) -> Select<entity::Post> {
        match list_type {
            ListType::ForCategory(id) => post.filter(entity::post::Column::CategoryId.eq(id)),
            ListType::ForTag(id) => post.filter(
                entity::post::Column::Id.in_subquery(
                    Query::select()
                        .column(entity::post_tag::Column::PostId)
                        .from(entity::PostTag)
                        .and_where(entity::post_tag::Column::TagId.eq(id))
                        .to_owned(),
                ),
            ),
            ListType::Full => post,
        }
    }
}

impl From<(PostModel, Option<CategoryModel>, Vec<TagModel>)> for Post {
    fn from(item: (PostModel, Option<CategoryModel>, Vec<TagModel>)) -> Post {
        let (post, category, tags) = item;

        Self::WithID(PostWithID {
            id: post.id,
            title: post.title,
            description: post.description,
            content: post.content,
            category: category.map(|category| Category::from(category)),
            tags: tags.into_iter().map(|tag| Tag::from(tag)).collect(),
            public_at: post.public_at,
        })
    }
}

impl From<deserializer::Post> for Post {
    fn from(post: deserializer::Post) -> Self {
        Self::WithoutID(PostWithoutID {
            title: post.title,
            description: post.description,
            content: post.content,
            category: post.category.map(|id| Category::from(id)),
            tags: post.tags.into_iter().map(|id| Tag::from(id)).collect(),
            public_at: if post.publish {
                Some(chrono::Local::now().naive_local())
            } else {
                None
            },
        })
    }
}
