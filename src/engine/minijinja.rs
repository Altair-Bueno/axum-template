use crate::TemplateEngine;

use super::Engine;

use axum::{http::StatusCode, response::IntoResponse};

#[cfg(feature = "minijinja")]
use minijinja::Environment;

#[cfg(feature = "minijinja-autoreload")]
use minijinja_autoreload::AutoReloader;

use thiserror::Error;

#[cfg(feature = "minijinja")]
impl TemplateEngine for Engine<Environment<'static>> {
    type Error = MinijinjaError;

    fn render<D: serde::Serialize>(&self, key: &str, data: D) -> Result<String, Self::Error> {
        let template = self.engine.get_template(key)?;
        let rendered = template.render(&data)?;

        Ok(rendered)
    }
}

#[cfg(feature = "minijinja-autoreload")]
impl TemplateEngine for Engine<AutoReloader> {
    type Error = MinijinjaError;

    fn render<D: serde::Serialize>(&self, key: &str, data: D) -> Result<String, Self::Error> {
        let reloader = self.engine.acquire_env()?;
        let template = reloader.get_template(key)?;
        // let template = self.engine.get_template(key)?;
        let rendered = template.render(&data)?;

        Ok(rendered)
    }
}

/// Error wrapper for [`minijinja::Error`]
#[derive(Error, Debug)]
pub enum MinijinjaError {
    /// See [`minijinja::Error`]
    #[error(transparent)]
    RenderError(#[from] minijinja::Error),
}

impl IntoResponse for MinijinjaError {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()).into_response()
    }
}
