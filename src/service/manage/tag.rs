use std::sync::Arc;

use axum::{
    extract::{Path, Query},
    Extension, Json,
};

use crate::{
    deserializer::{self, JsonRequest},
    middleware::state::State,
    model::{self, PostFor, PostListType},
    serializer::{self, post::PostType, ID},
    Response, Result,
};

use super::get_conn;

type JsonResponse<T> = Json<Response<T>>;

pub async fn list(
    Extension(state): Extension<Arc<State>>,
    Query(param): Query<deserializer::List>,
) -> Result<JsonResponse<serializer::TagList>> {
    let conn = get_conn(&state);

    let (models, total_page) = model::Tag::get_list(conn, param.page_size, param.page).await?;

    let models = models
        .into_iter()
        .map(|model| serializer::Tag::serialize(model))
        .collect();

    Ok(Json(Response::ok(serializer::TagList::from_vec(
        models, total_page,
    ))))
}

pub async fn get(
    Extension(state): Extension<Arc<State>>,
    Path(id): Path<i32>,
) -> Result<JsonResponse<serializer::Tag>> {
    let conn = get_conn(&state);

    let model = model::Tag::get_by_id(conn, id).await?;

    let model = serializer::Tag::serialize(model);

    Ok(Json(Response::ok(model)))
}

pub async fn add(
    Extension(state): Extension<Arc<State>>,
    JsonRequest(payload): JsonRequest<deserializer::Tag>,
) -> Result<JsonResponse<serializer::ID>> {
    let conn = get_conn(&state);

    let model = model::Tag::from(payload);

    let id = model::Tag::insert(conn, model).await?;

    Ok(Json(Response::ok(ID { id })))
}

pub async fn update(
    Extension(state): Extension<Arc<State>>,
    Path(id): Path<i32>,
    JsonRequest(payload): JsonRequest<deserializer::Tag>,
) -> Result<JsonResponse<serializer::ID>> {
    let conn = get_conn(&state);

    let model = model::Tag::from(payload);

    let id = model::Tag::update(conn, id, model).await?;

    Ok(Json(Response::ok(ID { id })))
}

pub async fn delete(
    Extension(state): Extension<Arc<State>>,
    Path(id): Path<i32>,
) -> Result<JsonResponse<()>> {
    let conn = get_conn(&state);

    model::Tag::delete(conn, id).await?;

    Ok(Json(Response::ok(())))
}
