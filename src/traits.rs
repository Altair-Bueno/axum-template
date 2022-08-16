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
/// #[derive(Debug)]
/// pub struct CustomEngine;
///
/// impl TemplateEngine for CustomEngine {
///     type Error = Infallible;   
///     fn render<S: Serialize>(&self, key: &str, data: S) -> Result<String, Self::Error> {
///         /* Render your template and return the result */
///         let result = "Hello world".into();
///         Ok(result)    
///     }
/// }
/// ```
///
/// > See the full working example [`custom_engine.rs`]
///
/// [`custom_engine.rs`]: https://github.com/Altair-Bueno/axum-template/blob/main/examples/custom_engine.rs
pub trait TemplateEngine {
    /// Error type returned if the engine is unable to process the data
    type Error: IntoResponse;

    /// Renders the template defined by the given key using the Serializable data
    fn render<S: Serialize>(&self, key: &str, data: S) -> Result<String, Self::Error>;
}
