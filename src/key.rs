use axum::{
    async_trait,
    extract::{FromRequest, MatchedPath, RequestParts},
};

use crate::TemplateError;

pub struct Key(pub String);

#[async_trait]
impl<B> FromRequest<B> for Key
where
    B: Send,
{
    type Rejection = TemplateError;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let path = req.extract::<MatchedPath>().await?.as_str().to_owned();
        Ok(Key(path))
    }
}
