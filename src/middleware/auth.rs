use std::fmt::Debug;
use std::sync::Arc;

use axum::response::IntoResponse;
use axum::{async_trait, RequestPartsExt, TypedHeader};
use axum::{
    extract::{FromRequest, FromRequestParts},
    headers::{authorization::Bearer, Authorization},
    http::{request::Parts, Request, StatusCode},
    middleware::Next,
    Extension,
};
use jwt_auth::Jwt;
use serde::{Deserialize, Serialize};

use crate::{Error, ErrorType};

use super::State;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub name: String,
    pub admin: bool,
    pub exp: usize,
}

pub async fn require_admin<B>(
    Extension(state): Extension<Arc<State>>,
    TypedHeader(auth): TypedHeader<Authorization<Bearer>>,
    request: Request<B>,
    next: Next<B>,
) -> impl IntoResponse
where
    B: Debug,
{
    let claims: Claims = state.jwt.verify(auth.token()).map_err(|err| {
        (
            StatusCode::UNAUTHORIZED,
            Error::new("Authorization", &err, ErrorType::Unauthorized),
        )
    })?;

    if claims.admin {
        let response = next.run(request).await;
        Ok(response)
    } else {
        Err((
            StatusCode::UNAUTHORIZED,
            Error::new(
                "Authorization",
                "Permission denied.",
                ErrorType::Unauthorized,
            ),
        ))
    }
}
