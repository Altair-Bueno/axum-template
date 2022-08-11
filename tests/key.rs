use anyhow::Ok;
use axum::{body::Body, http::Request, routing::get, Router};
use axum_template::Key;
use rstest::*;
use speculoos::*;
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
    let router: Router = Router::new().route(route, get(|Key(key): Key| async move { key }));

    let response = router
        .oneshot(Request::builder().uri(uri).body(Body::empty())?)
        .await?;

    let body = String::from_utf8(Vec::from(
        &hyper::body::to_bytes(response.into_body()).await?[..],
    ))?;
    assert_that!(body.as_str()).is_equal_to(route);

    Ok(())
}
