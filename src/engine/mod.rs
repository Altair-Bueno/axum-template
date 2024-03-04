//! Types that implement `TemplateEngine` for commonly used template engines
//!
//! > Note: each engine is guarded behind a feature with the same name
//!
//! # Table of contents
//!
//! - [`handlebars`](#handlebars)
//! - [`minijinja`](#minijinja)
//! - [`tera`](#tera)
//!
//! # `handlebars`
//!
//! ```no_run
#![doc = include_str!("../../examples/handlebars.rs")]
//! ```
//!
//! # `minijinja`
//!
//! ```no_run
#![doc = include_str!("../../examples/minijinja.rs")]
//! ```
//!
//! # `tera`
//!
//! ```no_run
#![doc = include_str!("../../examples/tera.rs")]
//! ```
//!

use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts},
    http::request::Parts,
};
use std::{convert::Infallible, fmt::Debug, sync::Arc};

#[cfg(feature = "handlebars")]
mod handlebars;
#[cfg(feature = "handlebars")]
pub use self::handlebars::*;

#[cfg(feature = "tera")]
mod tera;
#[cfg(feature = "tera")]
pub use self::tera::*;

#[cfg(any(feature = "minijinja", feature = "minijinja-autoreload"))]
mod minijinja;
#[cfg(any(feature = "minijinja", feature = "minijinja-autoreload"))]
pub use self::minijinja::*;

/// A wrapper type that implements [`TemplateEngine`] for multiple
/// commonly used engines. See [`crate::engine`] for detailed usage instructions
/// and examples
///
/// [`TemplateEngine`]: crate::TemplateEngine
#[derive(Debug, PartialEq, Eq)]
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

impl<E> Clone for Engine<E> {
    fn clone(&self) -> Self {
        Self {
            engine: self.engine.clone(),
        }
    }
}

impl<E> From<E> for Engine<E> {
    fn from(engine: E) -> Self {
        Self::new(engine)
    }
}

#[async_trait]
impl<ApplicationState, E> FromRequestParts<ApplicationState> for Engine<E>
where
    Self: Send + Sync + 'static + FromRef<ApplicationState>,
    ApplicationState: Send + Sync,
{
    type Rejection = Infallible;

    async fn from_request_parts(
        _: &mut Parts,
        state: &ApplicationState,
    ) -> Result<Self, Self::Rejection> {
        Ok(Self::from_ref(state))
    }
}
