use axum::{response::IntoResponse, routing::get, Router, Server};
use serde::Serialize;
use tera::Tera;
use tower::ServiceBuilder;

use axum_template::{engines::Engine, Render, Template};

#[derive(Debug, Serialize)]
struct Greeting {
    title: &'static str,
    content: &'static str,
}

async fn greeting(template: Template<Engine<Tera>>) -> impl IntoResponse {
    let stuff = Greeting {
        title: "This is a title",
        content: "hello world",
    };
    Render(template, stuff)
}

#[tokio::main]
pub async fn main() {
    let tera = Tera::new("examples/templates/tera/**/*.html").expect("Missing templates folder");

    let middleware = ServiceBuilder::new().layer(Engine::new(tera));

    let app = Router::new()
        .route("/greeting", get(greeting))
        .layer(middleware);

    Server::bind(&([127, 0, 0, 1], 8080).into())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
