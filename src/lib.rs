//! Placeholder
//!
//!

mod error;
mod render;
mod traits;

pub mod engines;

pub use error::TemplateError;
pub use render::{Render, RenderHtml};
pub use traits::TemplateEngine;
