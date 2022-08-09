use axum::{extract::Path, response::IntoResponse, routing::get, Router, Server};
use axum_template::{engines::Engine, Render, Template};
use handlebars::Handlebars;
use serde::Serialize;

type AppEngine = Engine<Handlebars<'static>>;

#[derive(Debug, Serialize)]
pub struct Person {
    name: String,
}

async fn person(template: Template<AppEngine>, Path(name): Path<String>) -> impl IntoResponse {
    let person = Person { name };

    Render(template, person)
}

#[tokio::main]
async fn main() {
    let hbs = Handlebars::new();

    let app = Router::new()
        .route("/:name", get(person))
        .layer(Engine::new(hbs));

    Server::bind(&([127, 0, 0, 1], 8080).into())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
