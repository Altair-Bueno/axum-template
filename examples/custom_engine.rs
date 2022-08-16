//! Creating your own custom engines
//!
//! Run the example using
//!
//! ```sh
//! cargo run --example=custom_engine
//! ```

use std::convert::Infallible;

use axum::{
    async_trait,
    extract::{rejection::ExtensionRejection, FromRequest},
    response::IntoResponse,
    routing::get,
    Extension, Router, Server,
};
use axum_template::{Key, RenderHtml, TemplateEngine};
use serde::Serialize;

// Clone is required by `axum::extract::Extension`
#[derive(Debug, Clone)]
pub struct CustomEngine;

impl TemplateEngine for CustomEngine {
    // See `std::convert::Infallible`
    type Error = Infallible;

    fn render<S: Serialize>(&self, key: &str, _: S) -> Result<String, Self::Error> {
        // This engine just returns the key
        Ok(key.to_owned())
    }
}

#[async_trait]
impl<B: Send> FromRequest<B> for CustomEngine {
    type Rejection = ExtensionRejection;

    async fn from_request(
        req: &mut axum::extract::RequestParts<B>,
    ) -> Result<Self, Self::Rejection> {
        let Extension(req) = req.extract().await?;
        Ok(req)
    }
}

async fn get_name(
    // Obtain the engine
    engine: AppEngine,
    // Extract the key
    Key(key): Key,
) -> impl IntoResponse {
    RenderHtml(key, engine, ())
}

type AppEngine = CustomEngine;

#[tokio::main]
async fn main() {
    let engine = CustomEngine;
    let app = Router::new()
        .route("/:name", get(get_name))
        // Share the engine using `axum::Extension`, or implement `tower::Layer`
        // manually for your engine
        .layer(Extension(engine));

    println!("See example: http://127.0.0.1:8080/example");
    Server::bind(&([127, 0, 0, 1], 8080).into())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
