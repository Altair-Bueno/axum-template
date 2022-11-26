use axum::{body::Body, http::Request, routing::get, Router};
use axum_template::Key;
use rstest::*;
use speculoos::prelude::*;
use tower::util::ServiceExt;

#[rstest]
#[case("/", "/")]
#[case("/:hello", "/world")]
#[case("/:hello", "/guys")]
#[trace]
#[tokio::test]
async fn key_extracts_from_request_route_path(
    #[case] route: &'static str,
    #[case] uri: &'static str,
) -> anyhow::Result<()> {
    let router: Router = Router::new().route(
        route,
        get(move |Key(key): Key| async move { assert_that!(key.as_str()).is_equal_to(route) }),
    );

    let _response = router
        .oneshot(Request::builder().uri(uri).body(Body::empty())?)
        .await?;

    Ok(())
}

#[rstest]
#[trace]
#[tokio::test]
async fn key_impl_asref_str() {
    fn inner(_: impl AsRef<str>) {}
    inner(Key("Some String".into()));
}
