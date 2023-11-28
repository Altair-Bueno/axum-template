//! Creating your own custom engines
//!
//! Run the example using
//!
//! ```sh
//! cargo run --example=custom_engine
//! ```

use std::{convert::Infallible, net::Ipv4Addr};

use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts},
    http::request::Parts,
    response::IntoResponse,
    routing::get,
    serve, Router,
};
use axum_template::{Key, RenderHtml, TemplateEngine};
use serde::Serialize;
use tokio::net::TcpListener;

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
impl<ApplicationState> FromRequestParts<ApplicationState> for CustomEngine
where
    Self: Send + Sync + 'static + FromRef<ApplicationState>,
    ApplicationState: Send + Sync,
{
    type Rejection = Infallible;

    async fn from_request_parts(
        _: &mut Parts,
        state: &ApplicationState,
    ) -> Result<Self, Self::Rejection> {
        Ok(Self::from_ref(state))
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

#[derive(Clone, FromRef)]
struct AppState {
    engine: AppEngine,
}

#[tokio::main]
async fn main() {
    let engine = CustomEngine;
    let app = Router::new()
        .route("/:name", get(get_name))
        // Share the engine using `axum::Extension`, or implement `tower::Layer`
        // manually for your engine
        .with_state(AppState { engine });

    println!("See example: http://127.0.0.1:8080/example");
    let listener = TcpListener::bind((Ipv4Addr::LOCALHOST, 8080))
        .await
        .unwrap();
    serve(listener, app.into_make_service()).await.unwrap();
}
