use axum::response::{Html, IntoResponse};
use serde::Serialize;

use crate::{template::Template, Key, TemplateEngine};

#[derive(Debug)]
pub struct Render<E, S>(pub Template<E>, pub S);

impl<E, S> IntoResponse for Render<E, S>
where
    E: TemplateEngine,
    S: Serialize,
{
    fn into_response(self) -> axum::response::Response {
        let Render(Template(Key(key), engine), data) = self;

        let result = engine.render(key.as_str(), data);

        match result {
            Ok(x) => Html(x).into_response(),
            Err(x) => x.into_response(),
        }
    }
}
