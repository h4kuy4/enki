use chrono::NaiveDateTime;
use serde::Serialize;

use super::Category;

use crate::model;
use crate::utils::md_to_html;

#[derive(Debug, Serialize)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub category: Category,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<NaiveDateTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
}

impl Post {
    pub fn serialize(model: model::Post) -> Self {
        let id = match model.id {
            Some(id) => id,
            None => {
                panic!("Post model id is None");
            }
        };

        Self {
            id,
            title: model.title,
            category: Category::serialize(model.category),
            date: model.public_at,
            content: Some(model.content),
            description: Some(model.description),
        }
    }

    pub fn without_content(mut self) -> Self {
        self.content = None;

        self
    }

    pub fn without_description(mut self) -> Self {
        self.description = None;

        self
    }

    pub fn render(mut self) -> Self {
        self.content = match self.content {
            Some(content) => Some(md_to_html::render(content)),
            None => None,
        };

        self
    }
}
