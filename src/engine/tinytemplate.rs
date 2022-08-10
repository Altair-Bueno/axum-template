use crate::TemplateEngine;

use super::Engine;

use axum::{http::StatusCode, response::IntoResponse};
use thiserror::Error;
use tinytemplate::TinyTemplate;

impl TemplateEngine for Engine<TinyTemplate<'static>> {
    type Error = TinyTemplateError;

    fn render<D: serde::Serialize>(&self, key: &str, data: D) -> Result<String, Self::Error> {
        let rendered = self.engine.render(key, &data)?;

        Ok(rendered)
    }
}

#[derive(Error, Debug)]
pub enum TinyTemplateError {
    #[error(transparent)]
    RenderError(#[from] tinytemplate::error::Error),
}

impl IntoResponse for TinyTemplateError {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::INTERNAL_SERVER_ERROR, self).into_response()
    }
}
