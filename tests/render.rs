use std::convert::Infallible;

use axum::response::IntoResponse;
use axum_template::{Render, RenderHtml, TemplateEngine};
use rstest::*;

#[derive(Debug, Clone, Default)]
pub struct MockEngine;

impl TemplateEngine for MockEngine {
    type Error = Infallible;

    fn render<S: serde::Serialize>(&self, _: &str, _: S) -> Result<String, Self::Error> {
        Ok("".to_owned())
    }
}

#[fixture]
fn engine() -> MockEngine {
    MockEngine::default()
}

#[rstest]
#[trace]
#[tokio::test]
async fn render_responds_with_text_plain(
    engine: impl TemplateEngine,
    #[values("")] key: &str,
    #[values(())] data: (),
) -> anyhow::Result<()> {
    let response = Render(key, engine, data).into_response();

    let (parts, _) = response.into_parts();
    let content_type = parts
        .headers
        .get(axum::http::header::CONTENT_TYPE)
        .unwrap()
        .as_bytes();

    assert_eq!(content_type, b"text/plain; charset=utf-8");
    Ok(())
}

#[rstest]
#[trace]
#[tokio::test]
async fn render_html_responds_with_text_html(
    engine: impl TemplateEngine,
    #[values("")] key: &str,
    #[values(())] data: (),
) -> anyhow::Result<()> {
    let response = RenderHtml(key, engine, data).into_response();

    let (parts, _) = response.into_parts();
    let content_type = parts
        .headers
        .get(axum::http::header::CONTENT_TYPE)
        .unwrap()
        .as_bytes();

    assert_eq!(content_type, b"text/html; charset=utf-8");
    Ok(())
}
