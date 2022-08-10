use axum::{extract::Path, response::IntoResponse, routing::get, Router, Server};
use axum_template::{engines::Engine, Key, RenderHtml};
use serde::Serialize;
use tera::Tera;

type AppEngine = Engine<Tera>;

#[derive(Debug, Serialize)]
pub struct Person {
    name: String,
}
async fn get_name(engine: AppEngine, Key(key): Key, Path(name): Path<String>) -> impl IntoResponse {
    let person = Person { name };

    RenderHtml(key, engine, person)
}

#[tokio::main]
async fn main() {
    let mut tera = Tera::default();
    tera.add_raw_template("/:name", "<h1>Hello Tera!</h1><p>{{name}}</p>")
        .unwrap();

    let app = Router::new()
        .route("/:name", get(get_name))
        .layer(Engine::new(tera));

    Server::bind(&([127, 0, 0, 1], 8080).into())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
