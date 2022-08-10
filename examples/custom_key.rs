//! Applying transformations to keys (prefixes and suffixes, for example)
//!
//! Run the example using
//!
//! ```sh
//! cargo run --examples=custom_key --features=tera
//! ```

use axum::{
    async_trait,
    extract::{FromRequest, MatchedPath, Path, RequestParts},
    response::IntoResponse,
    routing::get,
    Router, Server,
};
use axum_template::{engines::Engine, RenderHtml, TemplateError};
use serde::Serialize;
use tera::Tera;

// Because Tera::new loads an entire folder, we need to remove the `/` prefix
// and add a `.html` suffix. We can implement our own custom key extractor that
// transform the key
pub struct CustomKey(pub String);

#[async_trait]
impl<B> FromRequest<B> for CustomKey
where
    B: Send,
{
    type Rejection = TemplateError;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let key = req
            // axum_template::Key internally uses `axum::extract::MatchedPath`
            .extract::<MatchedPath>()
            .await?
            .as_str()
            .chars()
            // Remove the first character (/)
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

#[tokio::main]
async fn main() {
    // Tera allows loading an entire folder using glob patterns. This will load
    // our file examples/templates/tera/:name.html with the key :name.html
    let tera = Tera::new("examples/templates/tera/**/*.html").expect("Template folder not found");
    let app = Router::new()
        .route("/:name", get(get_name))
        .layer(Engine::new(tera));

    println!("See example: http://127.0.0.1:8080/example");
    Server::bind(&([127, 0, 0, 1], 8080).into())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
