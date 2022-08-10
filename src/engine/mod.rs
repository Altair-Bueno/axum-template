//! Types that implement `TemplateEngine` for commonly used template engines
//! from [crates.io](https://crates.io)
//! 
//! > Note: each engine is guarded behind a feature with the same name
//! 
//! # Table of contents
//! 
//! - [`handlebars`](#handlebars)
//! - [`minijinja`](#minijinja)
//! - [`tera`](#tera)
//! - [`tinytemplate`](#tinytemplate)
//! 
//! # `handlebars`
#![doc = concat!("```ignore\n",include_str!("../../examples/handlebars.rs"), "\n```")]
//! # `minijinja`
#![doc = concat!("```ignore\n",include_str!("../../examples/minijinja.rs"), "\n```")]
//! # `tera`
#![doc = concat!("```ignore\n",include_str!("../../examples/tera.rs"), "\n```")]
//! # `tinytemplate`
#![doc = concat!("```ignore\n",include_str!("../../examples/tinytemplate.rs"), "\n```")]

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
use crate::error::*;

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

/// A type that implements [`crate::TemplateEngine`] for common engines. See [`crate::engine`] 
/// for usage instructions
#[derive(Debug, Clone)]
pub struct Engine<E> {
    #[allow(dead_code)]
    engine: Arc<E>,
}

impl<E> Engine<E> {
    /// Creates a new [`Engine`] that wraps the given engine
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
