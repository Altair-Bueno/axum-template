#![allow(unused)]
use std::marker::PhantomData;

use axum::extract::FromRef;
use axum::extract::FromRequest;
use axum::extract::FromRequestParts;
use axum_template::engine::Engine;
use axum_template::TemplateEngine;
use rstest::*;

struct AssertImpl<E, S>(pub E, PhantomData<S>)
where
    E: Send + Sync + TemplateEngine + FromRef<S>;

#[cfg(feature = "tera")]
#[rstest]
fn engine_teras_assert_impl() {
    AssertImpl(Engine::new(tera::Tera::default()), Default::default());
}

#[cfg(feature = "handlebars")]
#[rstest]
fn engine_handlebars_assert_impl() {
    let phantom: PhantomData<()> = Default::default();
    AssertImpl(
        Engine::new(handlebars::Handlebars::new()),
        Default::default(),
    );
}

#[cfg(feature = "minijinja")]
#[rstest]
fn engine_minijinja_assert_impl() {
    let phantom: PhantomData<()> = Default::default();
    AssertImpl(
        Engine::new(minijinja::Environment::new()),
        Default::default(),
    );
}

#[cfg(feature = "minijinja-autoreload")]
#[rstest]
fn engine_minijinja_autoreload_assert_impl() {
    let phantom: PhantomData<()> = Default::default();
    let jinja = minijinja_autoreload::AutoReloader::new(move |_| Ok(minijinja::Environment::new()));
    AssertImpl(Engine::new(jinja), Default::default());
}
