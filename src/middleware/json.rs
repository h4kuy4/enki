use axum::{
    async_trait,
    body::HttpBody,
    extract::FromRequest,
    http::{Request, StatusCode},
    BoxError,
};

use serde::de::DeserializeOwned;

use crate::Error;

pub struct Json<T>(pub T);

#[async_trait]
impl<S, B, T> FromRequest<S, B> for Json<T>
where
    T: DeserializeOwned,
    B: Send + HttpBody + 'static,
    B::Data: Send,
    B::Error: Into<BoxError>,
    S: Send + Sync,
{
    type Rejection = (StatusCode, Error);

    async fn from_request(req: Request<B>, state: &S) -> Result<Self, Self::Rejection> {
        let axum::Json(payload) = axum::Json::<T>::from_request(req, state)
            .await
            .map_err(|err| (StatusCode::BAD_REQUEST, err.into()))?;

        Ok(Json(payload))
    }
}
