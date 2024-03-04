//! Simple usage of using `axum_template` with the `minijinja-autoreload` crate
//!
//! Run the example using
//!
//! ```sh
//! cargo run --example=minijinja-autoreload --features=minijinja,minijinja/loader,minijinja-autoreload,minijinja-autoreload/watch-fs
//! ```
use axum::{
    extract::{FromRef, Path},
    response::IntoResponse,
    routing::get,
    serve, Router,
};
use axum_template::{engine::Engine, RenderHtml};
use minijinja::{path_loader, Environment};
use minijinja_autoreload::AutoReloader;
use serde::Serialize;
use std::net::Ipv4Addr;
use std::path::PathBuf;
use tokio::net::TcpListener;

// Type alias for our engine. For this example, we are using Mini Jinja
type AppEngine = Engine<AutoReloader>;

#[derive(Debug, Serialize)]
pub struct Person {
    name: String,
}

async fn get_name(engine: AppEngine, Path(name): Path<String>) -> impl IntoResponse {
    let person = Person { name };

    RenderHtml("hello.html", engine, person)
}

// Define your application shared state
#[derive(Clone, FromRef)]
struct AppState {
    engine: AppEngine,
}

#[tokio::main]
async fn main() {
    // Set up the `minijinja` engine with the same route paths as the Axum router
    let jinja = AutoReloader::new(move |notifier| {
        let template_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("examples")
            .join("templates")
            .join("minijinja");
        let mut env = Environment::new();
        env.set_loader(path_loader(&template_path));
        notifier.set_fast_reload(true);
        notifier.watch_path(&template_path, true);
        Ok(env)
    });

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
