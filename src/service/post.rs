use std::sync::Arc;

use axum::{
    extract::{Path, Query},
    Extension, Json,
};

use crate::{deserializer, middleware::state::State, model, serializer, Response, Result};

use super::get_conn;

type JsonResponse<T> = Json<Response<T>>;

pub async fn list(
    Extension(state): Extension<Arc<State>>,
    Query(param): Query<deserializer::List>,
) -> Result<JsonResponse<serializer::PostList>> {
    let conn = get_conn(&state);

    let (models, total_page) = model::Post::get_list(conn, param.page_size, param.page).await?;

    let models = models
        .into_iter()
        .map(|model| serializer::Post::serialize(model).without_content())
        .collect();

    Ok(Json(Response::ok(serializer::PostList::from_vec(
        models, total_page,
    ))))
}

pub async fn get(
    Extension(state): Extension<Arc<State>>,
    Path(id): Path<i32>,
) -> Result<JsonResponse<serializer::Post>> {
    let conn = get_conn(&state);

    let model = model::Post::get_by_id(conn, id).await?;

    let model = serializer::Post::serialize(model)
        .without_description()
        .render();

    Ok(Json(Response::ok(model)))
}
