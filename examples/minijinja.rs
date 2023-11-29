//! Simple usage of using `axum_template` with the `minijinja` crate
//!
//! Run the example using
//!
//! ```sh
//! cargo run --example=minijinja --features=minijinja
//! ```
use std::net::Ipv4Addr;

use axum::{
    extract::{FromRef, Path},
    response::IntoResponse,
    routing::get,
    serve, Router,
};
use axum_template::{engine::Engine, Key, RenderHtml};
use minijinja::Environment;
use serde::Serialize;
use tokio::net::TcpListener;

// Type alias for our engine. For this example, we are using Mini Jinja
type AppEngine = Engine<Environment<'static>>;

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

// Define your application shared state
#[derive(Clone, FromRef)]
struct AppState {
    engine: AppEngine,
}

#[tokio::main]
async fn main() {
    // Set up the `minijinja` engine with the same route paths as the Axum router
    let mut jinja = Environment::new();
    jinja
        .add_template("/:name", "<h1>Hello Minijinja!</h1><p>{{name}}</p>")
        .unwrap();

    let app = Router::new()
        .route("/:name", get(get_name))
        // Create the application state
        .with_state(AppState {
            engine: Engine::from(jinja),
        });

    println!("See example: http://127.0.0.1:8080/example");

    let listener = TcpListener::bind((Ipv4Addr::LOCALHOST, 8080))
        .await
        .unwrap();
    serve(listener, app.into_make_service()).await.unwrap();
}
