//! Showcases nested routers and the `Key` extractor
//!
//! Run the example using
//!
//! ```sh
//! cargo run --example=nested --features=handlebars
//! ```

use std::net::Ipv4Addr;

use axum::{
    extract::{FromRef, Path},
    response::IntoResponse,
    routing::get,
    serve, Router,
};
use axum_template::{engine::Engine, Key, RenderHtml};
use handlebars::Handlebars;
use serde::Serialize;
use tokio::net::TcpListener;

type AppEngine = Engine<Handlebars<'static>>;

#[derive(Debug, Serialize)]
pub struct Person {
    name: String,
}

async fn get_name(engine: AppEngine, Key(key): Key, Path(name): Path<String>) -> impl IntoResponse {
    let person = Person { name };

    RenderHtml(key, engine, person)
}

#[derive(Clone, FromRef)]
struct AppState {
    engine: AppEngine,
}

#[tokio::main]
async fn main() {
    let mut hbs = Handlebars::new();
    hbs.register_template_string("/:name", "Simple {{ name }}")
        .unwrap();
    hbs.register_template_string("/nested/:name", "Nested {{name}}")
        .unwrap();

    let nested = Router::new().route("/:name", get(get_name));

    let app = Router::new()
        .route("/:name", get(get_name))
        .nest("/nested", nested)
        .with_state(AppState {
            engine: Engine::from(hbs),
        });

    println!("See example: http://127.0.0.1:8080/example");
    let listener = TcpListener::bind((Ipv4Addr::LOCALHOST, 8080))
        .await
        .unwrap();
    serve(listener, app.into_make_service()).await.unwrap();
}
