//! Simple usage of using `axum_template` with the `handlebars` crate
//!
//! Run the example using
//!
//! ```sh
//! cargo run --example=handlebars --features=handlebars
//! ```
use axum::{
    extract::{FromRef, Path},
    response::IntoResponse,
    routing::get,
    Router, Server,
};
use axum_template::{engine::Engine, Key, RenderHtml};
use handlebars::Handlebars;
use serde::Serialize;

// Type alias for our engine. For this example, we are using Handlebars
type AppEngine = Engine<Handlebars<'static>>;

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

#[derive(Clone, FromRef)]
struct AppState {
    engine: AppEngine,
}

#[tokio::main]
async fn main() {
    // Set up the Handlebars engine with the same route paths as the Axum router
    let mut hbs = Handlebars::new();
    hbs.register_template_string("/:name", "<h1>Hello HandleBars!</h1><p>{{name}}</p>")
        .unwrap();

    let app = Router::new()
        .route("/:name", get(get_name))
        // Add the Engine layer with your Handlebars instance
        .with_state(AppState {
            engine: Engine::from(hbs),
        });
    println!("See example: http://127.0.0.1:8080/example");
    Server::bind(&([127, 0, 0, 1], 8080).into())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
