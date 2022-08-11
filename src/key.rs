use axum::{
    async_trait,
    extract::{rejection::MatchedPathRejection, FromRequest, MatchedPath, RequestParts},
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
/// - [`axum::extract::MatchedPath`]
/// - Example: [`custom_key.rs`](examples/custom_key.rs)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Key(pub String);

#[async_trait]
impl<B> FromRequest<B> for Key
where
    B: Send,
{
    type Rejection = MatchedPathRejection;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let path = req.extract::<MatchedPath>().await?.as_str().to_owned();
        Ok(Key(path))
    }
}
