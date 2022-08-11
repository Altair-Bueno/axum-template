use axum::response::{Html, IntoResponse};
use serde::Serialize;

use crate::TemplateEngine;

/// Rendered template response
///
/// Responds to the request with the rendered template and
/// `text/plain; charset=utf-8` content-type
///
/// # Usage
///
/// ```
/// # use axum::{response::IntoResponse};
/// # use axum_template::{Render, TemplateEngine};
/// use serde::Serialize;
///
/// #[derive(Serialize)]
/// struct Person { /* */ }
///
/// async fn handler(
///     engine: impl TemplateEngine,
/// ) -> impl IntoResponse {
///     let key = "Template key".into();
///     let data = Person{ /* */ };
///     Render(key, engine, data)
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Render<E, S>(pub String, pub E, pub S);

impl<E, S> IntoResponse for Render<E, S>
where
    E: TemplateEngine,
    S: Serialize,
{
    fn into_response(self) -> axum::response::Response {
        let Render(key, engine, data) = self;

        let result = engine.render(key.as_str(), data);

        match result {
            Ok(x) => x.into_response(),
            Err(x) => x.into_response(),
        }
    }
}

/// Rendered Html response
///
/// Responds to the request with the rendered template and
/// `text/html` content-type
///
/// # Usage
///
/// ```
/// # use axum::{response::IntoResponse};
/// # use axum_template::{RenderHtml, TemplateEngine};
/// use serde::Serialize;
///
/// #[derive(Serialize)]
/// struct Person { /* */ }
///
/// async fn handler(
///     engine: impl TemplateEngine,
/// ) -> impl IntoResponse {
///     let key = "Template key".into();
///     let data = Person{ /* */ };
///     RenderHtml(key, engine, data)
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RenderHtml<E, S>(pub String, pub E, pub S);

impl<E, S> IntoResponse for RenderHtml<E, S>
where
    E: TemplateEngine,
    S: Serialize,
{
    fn into_response(self) -> axum::response::Response {
        let RenderHtml(key, engine, data) = self;

        let result = engine.render(key.as_str(), data);

        match result {
            Ok(x) => Html(x).into_response(),
            Err(x) => x.into_response(),
        }
    }
}

impl<E, S> From<Render<E, S>> for RenderHtml<E, S> {
    fn from(r: Render<E, S>) -> Self {
        let Render(a, b, c) = r;
        Self(a, b, c)
    }
}

impl<E, S> From<RenderHtml<E, S>> for Render<E, S> {
    fn from(r: RenderHtml<E, S>) -> Self {
        let RenderHtml(a, b, c) = r;
        Self(a, b, c)
    }
}
