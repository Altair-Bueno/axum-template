//! Creating your own custom engines
//!
//! Run the example using
//!
//! ```sh
//! cargo run --example=custom_engine
//! ```

use std::convert::Infallible;

use axum::{response::IntoResponse, routing::get, Extension, Router, Server};
use axum_template::{Key, RenderHtml, TemplateEngine};
use serde::Serialize;

// Only Clone is required
#[derive(Debug, Default, Clone)]
pub struct CustomEngine;

impl TemplateEngine for CustomEngine {
    // See `std::convert::Infallible`
    type Error = Infallible;

    fn render<S: Serialize>(&self, key: &str, _: S) -> Result<String, Self::Error> {
        // This engine just returns the key
        Ok(key.to_owned())
    }
}

async fn get_name(
    // Obtain the engine
    Extension(engine): Extension<CustomEngine>,
    // Extract the key
    Key(key): Key,
) -> impl IntoResponse {
    RenderHtml(key, engine, ())
}

#[tokio::main]
async fn main() {
    // Tera allows loading an entire folder using glob patterns. This will load
    // our file examples/templates/tera/:name.html with the key :name.html
    let engine = CustomEngine::default();
    let app = Router::new()
        .route("/:name", get(get_name))
        // Share the engine using `axum::Extension`, or implement `FromRequest`
        // manually for your engine
        .layer(Extension(engine));

    println!("See example: http://127.0.0.1:8080/example");
    Server::bind(&([127, 0, 0, 1], 8080).into())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
