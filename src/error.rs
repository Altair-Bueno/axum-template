use axum::http::StatusCode;
use axum::response::IntoResponse;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TemplateError {
    #[error("{0}")]
    MissingKey(String),
    #[error("{0}")]
    MissingEngine(String),
}

impl IntoResponse for TemplateError {
    fn into_response(self) -> axum::response::Response {
        match self {
            TemplateError::MissingKey(x) => (StatusCode::INTERNAL_SERVER_ERROR, x).into_response(),
            TemplateError::MissingEngine(x) => {
                (StatusCode::INTERNAL_SERVER_ERROR, x).into_response()
            }
        }
    }
}
