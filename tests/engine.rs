#![allow(unused)]
use std::marker::PhantomData;

use axum::extract::FromRequest;
use axum_template::engine::Engine;
use axum_template::TemplateEngine;
use rstest::*;

struct AssertImpl<E, B>(pub E, pub PhantomData<B>)
where
    E: Send + Sync + TemplateEngine + FromRequest<B>,
    // Body type required by FromRequest
    B: Send;

#[cfg(feature = "tera")]
#[rstest]
fn engine_teras_assert_impl() {
    let phantom: PhantomData<()> = Default::default();
    AssertImpl(Engine::new(tera::Tera::default()), phantom);
}

#[cfg(feature = "handlebars")]
#[rstest]
fn engine_handlebars_assert_impl() {
    let phantom: PhantomData<()> = Default::default();
    AssertImpl(Engine::new(handlebars::Handlebars::new()), phantom);
}

#[cfg(feature = "minijinja")]
#[rstest]
fn engine_minijinja_assert_impl() {
    let phantom: PhantomData<()> = Default::default();
    AssertImpl(Engine::new(minijinja::Environment::new()), phantom);
}
