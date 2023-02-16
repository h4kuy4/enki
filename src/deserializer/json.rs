use axum::{
    async_trait,
    extract::{FromRequest, RequestParts},
    http::StatusCode,
    BoxError,
};

use serde::de::DeserializeOwned;

use crate::Error;

pub struct Json<T>(pub T);

#[async_trait]
impl<B, T> FromRequest<B> for Json<T>
where
    B: axum::body::HttpBody + Send,
    T: DeserializeOwned,
    B::Data: Send,
    B::Error: Into<BoxError>,
{
    type Rejection = (StatusCode, Error);

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        match axum::Json::<T>::from_request(req).await {
            Ok(value) => Ok(Self(value.0)),
            Err(err) => Err((StatusCode::BAD_REQUEST, err.into())),
        }
    }
}
