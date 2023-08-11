use std::{str::FromStr, sync::Arc};

use axum::{
    extract::{Path, Query},
    Extension, Json,
};
use chrono::NaiveDateTime;
use rss::{ChannelBuilder, Item, ItemBuilder};

use crate::{deserializer, middleware::state::State, model, serializer, Response, Result};

use super::get_conn;

pub async fn feed(Extension(state): Extension<Arc<State>>) -> Result<String> {
    let conn = get_conn(&state);

    let (models, _) = model::Post::get_list(conn, None, None).await?;

    let items: Vec<Item> = models
        .into_iter()
        .map(|model| {
            ItemBuilder::default()
                .title(Some(model.title))
                .description(Some(model.description))
                .link(Some(format!(
                    "https://www.hakuya.work/post/{}",
                    model.id.unwrap()
                )))
                .pub_date(Some(model.public_at.unwrap().to_string()))
                .build()
        })
        .collect();

    let channel = ChannelBuilder::default()
        .title("白夜的幻想乡缘起".to_string())
        .link("https://www.hakuya.work".to_string())
        .items(items)
        .build();

    Ok(channel.to_string())
}
