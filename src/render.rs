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
///     let key = "Template key";
///     let data = Person{ /* */ };
///     Render(key, engine, data)
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
#[must_use]
pub struct Render<K, E, S>(pub K, pub E, pub S);

impl<K, E, S> IntoResponse for Render<K, E, S>
where
    E: TemplateEngine,
    S: Serialize,
    K: AsRef<str>,
{
    fn into_response(self) -> axum::response::Response {
        let Render(key, engine, data) = self;

        let result = engine.render(key.as_ref(), data);

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
///     let key = "Template key";
///     let data = Person{ /* */ };
///     RenderHtml(key, engine, data)
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
#[must_use]
pub struct RenderHtml<K, E, S>(pub K, pub E, pub S);

impl<K, E, S> IntoResponse for RenderHtml<K, E, S>
where
    E: TemplateEngine,
    S: Serialize,
    K: AsRef<str>,
{
    fn into_response(self) -> axum::response::Response {
        let RenderHtml(key, engine, data) = self;

        let result = engine.render(key.as_ref(), data);

        match result {
            Ok(x) => Html(x).into_response(),
            Err(x) => x.into_response(),
        }
    }
}

impl<K, E, S> From<Render<K, E, S>> for RenderHtml<K, E, S> {
    fn from(r: Render<K, E, S>) -> Self {
        let Render(a, b, c) = r;
        Self(a, b, c)
    }
}

impl<K, E, S> From<RenderHtml<K, E, S>> for Render<K, E, S> {
    fn from(r: RenderHtml<K, E, S>) -> Self {
        let RenderHtml(a, b, c) = r;
        Self(a, b, c)
    }
}

impl<K, E, S> From<(K, E, S)> for Render<K, E, S> {
    fn from((k, e, s): (K, E, S)) -> Self {
        Self(k, e, s)
    }
}

impl<K, E, S> From<(K, E, S)> for RenderHtml<K, E, S> {
    fn from((k, e, s): (K, E, S)) -> Self {
        Self(k, e, s)
    }
}
