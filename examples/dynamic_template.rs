//! Simple usage of using `axum_template` with the `handlebars` crate
//!
//! Run the example using
//!
//! ```sh
//! cargo run --example=dynamic_template --features=handlebars
//! ```
use axum::{extract::FromRef, response::IntoResponse, routing::get, Router, Server};
use axum_template::{engine::Engine, RenderHtml};
use handlebars::Handlebars;

// Type alias for our engine. For this example, we are using Handlebars
type AppEngine = Engine<Handlebars<'static>>;

async fn get_luck(
    // Obtain the engine
    engine: AppEngine,
) -> impl IntoResponse {
    // Anything that can be coerced to &str can be used as Key.
    let key = if rand::random::<u8>() % 6 == 0 {
        "lucky"
    } else {
        "unlucky"
    };
    RenderHtml(key, engine, &())
}

// Define your application shared state
#[derive(Clone, FromRef)]
struct AppState {
    engine: AppEngine,
}

#[tokio::main]
async fn main() {
    // Set up the Handlebars engine with the same route paths as the Axum router
    let mut hbs = Handlebars::new();
    hbs.register_template_string("lucky", "<h1>Winner winner chicken dinner</h1>")
        .unwrap();
    hbs.register_template_string("unlucky", "<h1>Try again!</h1>")
        .unwrap();

    let app = Router::new()
        .route("/example", get(get_luck))
        // Create the application state
        .with_state(AppState {
            engine: Engine::from(hbs),
        });
    println!("See example: http://127.0.0.1:8080/example");
    Server::bind(&([127, 0, 0, 1], 8080).into())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
