use chrono::NaiveDateTime;
use serde::Serialize;

use super::{Category, Tag};

use crate::model;
use crate::utils::md_to_html;

#[derive(Debug, Serialize)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub category: Option<Category>,
    pub tags: Vec<Tag>,
    pub public_at: Option<NaiveDateTime>,

    #[serde(flatten)]
    pub content: Content,
}

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum Content {
    Content {
        content: String,
    },
    Description {
        description: String,
    },
    All {
        content: String,
        description: String,
    },
}

#[derive(Debug)]
pub enum PostType {
    ContentOnly,
    DescriptionOnly,
    Full,
}

impl Post {
    pub fn serialize(model: model::Post, post_type: PostType) -> Self {
        let model = match model {
            model::Post::WithoutID(_) => {
                log::error!("Post Model: Wrong model!");
                panic!()
            }
            model::Post::WithID(model) => model,
        };

        Self {
            id: model.id,
            title: model.title,
            category: model.category.map(|model| Category::serialize(model)),
            public_at: model.public_at,
            tags: model
                .tags
                .into_iter()
                .map(|model| Tag::serialize(model))
                .collect(),
            content: match post_type {
                PostType::Full => Content::All {
                    content: model.content,
                    description: model.description,
                },
                PostType::ContentOnly => Content::Content {
                    content: model.content,
                },
                PostType::DescriptionOnly => Content::Description {
                    description: model.description,
                },
            },
        }
    }

    pub fn render(mut self) -> Self {
        self.content = match self.content {
            Content::All {
                content,
                description,
            } => Content::All {
                content: md_to_html::render(content),
                description,
            },

            Content::Content { content } => Content::Content {
                content: md_to_html::render(content),
            },

            Content::Description { description: _ } => {
                log::error!("Post Serializer: Content field not found!");
                panic!()
            }
        };

        return self;
    }
}
