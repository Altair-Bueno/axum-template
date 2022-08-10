use axum::response::IntoResponse;
use serde::Serialize;

/// An abstraction over different templating engines
///
/// # Implementing custom engines
///
/// ```
/// # use axum_template::TemplateEngine;
/// # use serde::Serialize;
/// # use std::convert::Infallible;
///
/// #[derive(Debug, Default, Clone)]
/// pub struct CustomEngine;
///
/// impl TemplateEngine for CustomEngine {
///     type Error = Infallible;   
///     fn render<S: Serialize>(&self, key: &str, data: S) -> Result<String, Self::Error> {
///         Ok(key.to_owned())    
///     }
/// }
/// ```
///
/// > See example [`custom_engine.rs`](examples/custom_engine.rs)
pub trait TemplateEngine {
    /// Error type returned if the engine is unable to process the data
    type Error: IntoResponse;

    /// Renders the template defined by the given key using the Serializable data
    fn render<S: Serialize>(&self, key: &str, data: S) -> Result<String, Self::Error>;
}
