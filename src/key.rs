use axum::{
    extract::{
        rejection::MatchedPathRejection, FromRequestParts, MatchedPath, OptionalFromRequestParts,
    },
    http::request::Parts,
    RequestPartsExt,
};

/// Extracts matched path of the request
///
/// # Usage
///
/// ```
/// # use axum::{response::IntoResponse, Router, routing::get};
/// # use axum_template::Key;
/// async fn handler(
///     Key(key): Key
/// ) -> impl IntoResponse
/// {
///     key
/// }
///
/// let router: Router<()> = Router::new()
///     // key == "/some/route"
///     .route("/some/route", get(handler))
///     // key == "/{dynamic}"
///     .route("/{dynamic}", get(handler));
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

impl<S> OptionalFromRequestParts<S> for Key
where
    S: Send + Sync,
{
    type Rejection = <MatchedPath as OptionalFromRequestParts<S>>::Rejection;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &S,
    ) -> Result<Option<Self>, Self::Rejection> {
        let path =
            <MatchedPath as OptionalFromRequestParts<S>>::from_request_parts(parts, state).await?;
        Ok(path.map(|path| Key(path.as_str().to_owned())))
    }
}

impl AsRef<str> for Key {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

impl From<String> for Key {
    fn from(s: String) -> Self {
        Self(s)
    }
}
