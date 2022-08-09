use crate::{error::TemplateError, traits::TemplateEngine, Key};
use async_trait::async_trait;
use axum::extract::{FromRequest, RequestParts};

#[derive(Debug)]
pub struct Template<E>(pub Key, pub E);

#[async_trait]
impl<B, E> FromRequest<B> for Template<E>
where
    B: Send,
    E: TemplateEngine + Send + Sync + Clone + FromRequest<B, Rejection = TemplateError> + 'static,
{
    type Rejection = E::Rejection;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let key = req.extract::<Key>().await?;
        let engine = req.extract::<E>().await?;

        Ok(Template(key, engine))
    }
}
