use axum::{
    async_trait,
    extract::{rejection::MatchedPathRejection, FromRequest, MatchedPath, Path, RequestParts},
    response::IntoResponse,
    routing::get,
    Router, Server,
};
use axum_template::{engines::Engine, RenderHtml};
use serde::Serialize;
use tera::Tera;

pub struct Template(pub String);

#[async_trait]
impl<B> FromRequest<B> for Template
where
    B: Send, // required by `async_trait`
{
    type Rejection = MatchedPathRejection;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let path = req.extract::<MatchedPath>().await?;
        // Remove the first / and append .html
        let path = path
            .as_str()
            .chars()
            .skip(1)
            .chain(".html".chars())
            .collect();
        Ok(Template(path))
    }
}

type AppEngine = Engine<Tera>;

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
    let tera = Tera::new("examples/templates/tera/**/*.html").expect("Template folder not found");
    let app = Router::new()
        .route("/:name", get(get_name))
        .layer(Engine::new(tera));

    Server::bind(&([127, 0, 0, 1], 8080).into())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
