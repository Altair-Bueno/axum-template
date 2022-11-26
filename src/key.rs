use axum::{
    async_trait,
    extract::{rejection::MatchedPathRejection, FromRequestParts, MatchedPath},
    http::request::Parts,
    RequestPartsExt,
};

/// Extracts matched path of the request
///
/// # Usage
///
/// ```
/// # use axum::{response::IntoResponse, Server, Router, routing::get};
/// # use axum_template::Key;
/// async fn handler(
///     Key(key): Key
/// ) -> impl IntoResponse
/// {
///     key
/// }
///
/// let router = Router::new()
///     // key == "/some/route"
///     .route("/some/route", get(handler))
///     // key == "/:dynamic"
///     .route("/:dynamic", get(handler));
///
/// # async {
/// axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
///     .serve(router.into_make_service())
///     .await
///     .expect("server failed");
/// # };
/// ```
///
/// # Additional resources
///
/// - [`MatchedPath`]
/// - Example: [`custom_key.rs`]
///
/// [`MatchedPath`]: axum::extract::MatchedPath
/// [`custom_key.rs`]: https://github.com/Altair-Bueno/axum-template/blob/main/examples/custom_key.rs
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Key(pub String);

#[async_trait]
impl<S> FromRequestParts<S> for Key
where
    S: Send + Sync,
{
    type Rejection = MatchedPathRejection;

    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        let path = parts.extract::<MatchedPath>().await?.as_str().to_owned();
        Ok(Key(path))
    }
}

impl From<String> for Key {
    fn from(s: String) -> Self {
        Self(s)
    }
}
