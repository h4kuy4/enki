use std::sync::Arc;

use axum::{Extension, Json};

use crate::{middleware::state::State, model, serializer, Response, Result};

use super::get_conn;

type JsonResponse<T> = Json<Response<T>>;

pub async fn list(
    Extension(state): Extension<Arc<State>>,
) -> Result<JsonResponse<serializer::FriendList>> {
    let conn = get_conn(&state);

    let models = model::Friend::get_list(conn).await?;

    let models = models
        .into_iter()
        .map(|model| serializer::Friend::serialize(model))
        .collect();

    Ok(Json(Response::ok(serializer::FriendList::from_vec(
        models, 0,
    ))))
}
