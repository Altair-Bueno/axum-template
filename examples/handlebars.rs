use axum::{
    async_trait,
    extract::{rejection::MatchedPathRejection, FromRequest, MatchedPath, Path, RequestParts},
    response::IntoResponse,
    routing::get,
    Router, Server,
};
use axum_template::{engines::Engine, RenderHtml};
use handlebars::Handlebars;
use serde::Serialize;

pub struct Template(pub String);

#[async_trait]
impl<B> FromRequest<B> for Template
where
    B: Send,
{
    type Rejection = MatchedPathRejection;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let path = req.extract::<MatchedPath>().await?.as_str().to_owned();
        Ok(Template(path))
    }
}

type AppEngine = Engine<Handlebars<'static>>;

#[derive(Debug, Serialize)]
pub struct Person {
    name: String,
}

async fn get_name(
    engine: AppEngine,
    Template(template): Template,
    Path(name): Path<String>,
) -> impl IntoResponse {
    let person = Person { name };

    RenderHtml(template, engine, person)
}

#[tokio::main]
async fn main() {
    let mut hbs = Handlebars::new();
    hbs.register_template_string("/:name", "<h1>Hello HandleBars!</h1><p>{{name}}</p>")
        .unwrap();

    let app = Router::new()
        .route("/:name", get(get_name))
        .layer(Engine::new(hbs));

    Server::bind(&([127, 0, 0, 1], 8080).into())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
