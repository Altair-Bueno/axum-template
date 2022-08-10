use axum::{
    async_trait,
    extract::{FromRequest, RequestParts},
    Extension,
};
use std::sync::Arc;
use tower_http::add_extension::AddExtension;
use tower_layer::Layer;

#[cfg(feature = "handlebars")]
mod handlebars;
use crate::error::TemplateError;

#[cfg(feature = "handlebars")]
pub use self::handlebars::*;

#[cfg(feature = "tera")]
mod tera;
#[cfg(feature = "tera")]
pub use self::tera::*;

#[cfg(feature = "tinytemplate")]
mod tinytemplate;
#[cfg(feature = "tinytemplate")]
pub use self::tinytemplate::*;

#[cfg(feature = "minijinja")]
mod minijinja;
#[cfg(feature = "minijinja")]
pub use self::minijinja::*;

#[derive(Debug, Clone)]
pub struct Engine<E> {
    engine: Arc<E>,
}

impl<E> Engine<E> {
    pub fn new(engine: E) -> Self {
        let engine = Arc::new(engine);
        Self { engine }
    }
}

impl<E> From<E> for Engine<E> {
    fn from(engine: E) -> Self {
        Self::new(engine)
    }
}

impl<S, E> Layer<S> for Engine<E>
where
    E: Clone,
{
    type Service = AddExtension<S, Self>;

    fn layer(&self, inner: S) -> Self::Service {
        AddExtension::new(inner, self.clone())
    }
}

#[async_trait]
impl<B, E> FromRequest<B> for Engine<E>
where
    Self: Clone + Send + Sync + 'static,
    B: Send,
{
    type Rejection = TemplateError;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let Extension(engine) = req.extract::<Extension<Self>>().await.map_err(|_| {
            TemplateError::MissingEngine(format!(
                "Template engine missing. See documentation for {}",
                std::any::type_name::<Self>()
            ))
        })?;

        Ok(engine)
    }
}
