use std::sync::Arc;

use axum::{Extension, Json};

use crate::{
    deserializer,
    middleware::{Claims, JsonRequest, State},
    model, serializer, Error, ErrorType, Response, Result,
};

type JsonResponse<T> = Json<Response<T>>;

pub async fn login(
    Extension(state): Extension<Arc<State>>,
    JsonRequest(payload): JsonRequest<deserializer::Account>,
) -> Result<JsonResponse<serializer::Jwt>> {
    if state.account.verify(&payload.user_name, &payload.password) {
        let token = state
            .jwt
            .sign(Claims {
                name: payload.user_name,
                admin: true,
                exp: 10000000000000,
            })
            .map_err(|_| {
                Error::new(
                    "Authorization",
                    "Sign the jwt token failed",
                    ErrorType::ServerError,
                )
            })?;
        Ok(Json(Response::ok(serializer::Jwt::from(token))))
    } else {
        Err(Error::new(
            "Authorization",
            "User name or password error",
            ErrorType::Unauthorized,
        ))
    }
}
