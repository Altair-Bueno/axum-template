use crate::TemplateEngine;

use super::Engine;

use axum::{http::StatusCode, response::IntoResponse};
use handlebars::Handlebars;
use thiserror::Error;

impl TemplateEngine for Engine<Handlebars<'static>> {
    type Error = HandlebarsError;

    fn render<D: serde::Serialize>(&self, key: &str, data: D) -> Result<String, Self::Error> {
        let rendered = self.engine.render(key, &data)?;

        Ok(rendered)
    }
}

/// Error wrapper for [`handlebars::RenderError`]
#[derive(Error, Debug)]
pub enum HandlebarsError {
    /// See [`handlebars::RenderError`]
    #[error(transparent)]
    RenderError(#[from] handlebars::RenderError),
}

impl IntoResponse for HandlebarsError {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()).into_response()
    }
}
