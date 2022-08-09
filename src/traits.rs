use axum::response::IntoResponse;
use serde::Serialize;

pub trait TemplateEngine {
    type Error: IntoResponse;

    fn render<S: Serialize>(&self, key: &str, data: S) -> Result<String, Self::Error>;
}
