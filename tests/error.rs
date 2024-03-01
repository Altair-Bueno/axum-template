//! Regression tests for stack overflow on IntoResponse calls for error types
//!
//! See https://github.com/Altair-Bueno/axum-template/issues/8

#![allow(unused)]

use axum::response::IntoResponse;
use axum_template::engine::Engine;
use axum_template::Render;
use rstest::*;

#[cfg(feature = "tera")]
#[rstest]
#[trace]
#[tokio::test]
async fn tera_error_into_response_check_infinite_recursion() -> anyhow::Result<()> {
    let engine = tera::Tera::new("./*.nothing")?;
    let engine = Engine::new(engine);
    let data = ();
    _ = Render("", engine, data).into_response();
    Ok(())
}

#[cfg(feature = "handlebars")]
#[rstest]
#[trace]
#[tokio::test]
async fn handlebars_error_into_response_check_infinite_recursion() -> anyhow::Result<()> {
    let engine = handlebars::Handlebars::new();
    let engine = Engine::new(engine);
    let data = ();
    _ = Render("", engine, data).into_response();
    Ok(())
}

#[cfg(feature = "minijinja")]
#[rstest]
#[trace]
#[tokio::test]
async fn minijinja_error_into_response_check_infinite_recursion() -> anyhow::Result<()> {
    let engine = minijinja::Environment::new();
    let engine = Engine::new(engine);
    let data = ();
    _ = Render("", engine, data).into_response();
    Ok(())
}

#[cfg(feature = "minijinja-autoreload")]
#[rstest]
#[trace]
#[tokio::test]
async fn minijinja_autoreload_error_into_response_check_infinite_recursion() -> anyhow::Result<()> {
    let jinja = minijinja_autoreload::AutoReloader::new(move |_| Ok(minijinja::Environment::new()));
    let engine = Engine::new(jinja);
    let data = ();
    _ = Render("", engine, data).into_response();
    Ok(())
}
