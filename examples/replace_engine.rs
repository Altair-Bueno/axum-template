//! Uses different engines based on the enabled features
//!
//! Run the example using
//!
//! ```sh
//! # For minijinja
//! cargo run --example=replace_engine --features=minijinja
//! # For handlebars
//! cargo run --example=replace_engine --features=handlebars
//! ```
use axum::{extract::{Path}, response::IntoResponse, routing::get, Router, Server};
use axum_template::{engine::Engine, Key, RenderHtml};
use serde::Serialize;

const NAME_TEMPLATE: &str = "<h1>This example switches engines!</h1><p>{{name}}</p>";

#[cfg(feature = "minijinja")]
type AppEngine = Engine<minijinja::Environment<'static>>;
#[cfg(feature = "handlebars")]
type AppEngine = Engine<handlebars::Handlebars<'static>>;

#[derive(Debug, Serialize)]
pub struct Person {
    name: String,
}

async fn get_name(
    // Obtain the engine
    engine: AppEngine,
    // Extract the key
    Key(key): Key,
    Path(name): Path<String>,
) -> impl IntoResponse {
    let person = Person { name };

    RenderHtml(key, engine, person)
}

#[tokio::main]
async fn main() {
    let engine = get_engine();
    let app = Router::new()
        .route("/:name", get(get_name))
        .layer(Engine::new(engine));

    println!("See example: http://127.0.0.1:8080/example");
    Server::bind(&([127, 0, 0, 1], 8080).into())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[cfg(feature = "handlebars")]
fn get_engine() -> handlebars::Handlebars<'static>  {
    let mut hbs = handlebars::Handlebars::new();
    hbs.register_template_string("/:name", NAME_TEMPLATE).unwrap();
    hbs
}

#[cfg(feature = "minijinja")]
fn get_engine() -> minijinja::Environment<'static> {
    let mut jinja = minijinja::Environment::new();
    jinja
        .add_template("/:name", NAME_TEMPLATE)
        .unwrap();
    jinja
}