//! Applying transformations to keys (prefixes and suffixes, for example)
//!
//! Run the example using
//!
//! ```sh
//! cargo run --example=custom_key --features=tera
//! ```

use std::net::Ipv4Addr;

use axum::{
    async_trait,
    extract::{rejection::MatchedPathRejection, FromRef, FromRequestParts, MatchedPath, Path},
    http::request::Parts,
    response::IntoResponse,
    routing::get,
    serve, RequestPartsExt, Router,
};
use axum_template::{engine::Engine, RenderHtml};
use serde::Serialize;
use tera::Tera;
use tokio::net::TcpListener;

// Because Tera::new loads an entire folder, we need to remove the `/` prefix
// and add a `.html` suffix. We can implement our own custom key extractor that
// transform the key
pub struct CustomKey(pub String);

#[async_trait]
impl<S> FromRequestParts<S> for CustomKey
where
    S: Send + Sync,
{
    type Rejection = MatchedPathRejection;

    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        let key = parts
            // `axum_template::Key` internally uses `axum::extract::MatchedPath`
            .extract::<MatchedPath>()
            .await?
            .as_str()
            // Cargo doesn't allow `:` as a file name
            .replace(":", "$")
            .chars()
            // Remove the first character `/`
            .skip(1)
            // Add the `.html` suffix
            .chain(".html".chars())
            .collect();
        Ok(CustomKey(key))
    }
}

// Type alias for our engine. For this example, we are using Tera
type AppEngine = Engine<Tera>;

#[derive(Debug, Serialize)]
pub struct Person {
    name: String,
}

async fn get_name(
    // Obtain the engine
    engine: AppEngine,
    // Extract the custom key
    CustomKey(template): CustomKey,
    Path(name): Path<String>,
) -> impl IntoResponse {
    let person = Person { name };

    RenderHtml(template, engine, person)
}

#[derive(Clone, FromRef)]
struct AppState {
    engine: AppEngine,
}

#[tokio::main]
async fn main() {
    // Tera allows loading an entire folder using glob patterns. This will load
    // our file examples/templates/tera/$name.html with the key $name.html
    let tera = Tera::new("examples/templates/tera/**/*.html").expect("Template folder not found");
    let app = Router::new()
        .route("/:name", get(get_name))
        .with_state(AppState {
            engine: Engine::from(tera),
        });

    println!("See example: http://127.0.0.1:8080/example");
    let listener = TcpListener::bind((Ipv4Addr::LOCALHOST, 8080))
        .await
        .unwrap();
    serve(listener, app.into_make_service()).await.unwrap();
}
