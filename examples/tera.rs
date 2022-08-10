//! Simple usage of using `axum_template` with the `tera` crate
//!
//! Run the example using
//!
//! ```sh
//! cargo run --example=tera --features=tera
//! ```
use axum::{extract::Path, response::IntoResponse, routing::get, Router, Server};
use axum_template::{engine::Engine, Key, RenderHtml};
use serde::Serialize;
use tera::Tera;

// Type alias for our engine. For this example, we are using Tera
type AppEngine = Engine<Tera>;

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
    // Set up the Tera engine with the same route paths as the Axum router
    let mut tera = Tera::default();
    tera.add_raw_template("/:name", "<h1>Hello Tera!</h1><p>{{name}}</p>")
        .unwrap();

    let app = Router::new()
        .route("/:name", get(get_name))
        // Add the Engine layer with your Tera instance
        .layer(Engine::new(tera));

    println!("See example: http://127.0.0.1:8080/example");
    Server::bind(&([127, 0, 0, 1], 8080).into())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
