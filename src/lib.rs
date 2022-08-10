//! Placeholder
//!
//!
//!

#![warn(missing_docs)]
#![warn(missing_doc_code_examples)]

mod error;
mod key;
mod render;
mod traits;

pub mod engine;

pub use error::TemplateError;
pub use key::Key;
pub use render::{Render, RenderHtml};
pub use traits::TemplateEngine;
