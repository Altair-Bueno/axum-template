use axum::{
    extract::{FromRequest, RequestParts},
    response::IntoResponse,
    Extension,
};

use crate::error::TemplateError;
use async_trait::async_trait;

#[derive(Debug, Clone)]
pub struct Key(pub String);

#[async_trait]
impl<B> FromRequest<B> for Key
where
    B: Send,
{
    type Rejection = TemplateError;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let Extension(key) = req.extract::<Extension<Key>>().await.map_err(|_| {
            TemplateError::MissingKey(format!(
                "Template key missing. See {}",
                std::any::type_name::<Key>()
            ))
        })?;
        Ok(key)
    }
}

impl IntoResponse for Key {
    fn into_response(self) -> axum::response::Response {
        Extension(self).into_response()
    }
}
