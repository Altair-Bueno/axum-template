//! Layers, extractors and template engine wrappers for
//! [axum](https://github.com/tokio-rs/axum) based Web MVC applications
//!
//! # Getting started
//!
//! ## `Cargo.toml`
//!
//! ```toml
//! [dependencies]
//! axum-template = "0.1.0"
//! ```
//!
//! The [`engine`] module contains detailed usage examples for each of the
//! supported template engines.
//!
//! If you plan using an unsupported engine, check the [`TemplateEngine`] docs
//!
//! # Available features
//!
//! - `handlebars`: Enables [`handlebars`](https://crates.io/crates/handlebars) support
//! - `minijinja`: Enables [`minijinja`](https://crates.io/crates/minijinja) support
//! - `tera`: Enables [`tera`](https://crates.io/crates/tera) support
//!
//! # Useful links
//!
//! - [Documentation](https://docs.rs/axum-template)
//! - [Examples](https://github.com/Altair-Bueno/axum-template/examples)
//! - [Source code](https://github.com/Altair-Bueno/axum-template)
//!
//! [`engine`]: crate::engine
//! [`TemplateEngine`]: crate::TemplateEngine

#![warn(
    clippy::all,
    clippy::dbg_macro,
    clippy::todo,
    clippy::empty_enum,
    clippy::enum_glob_use,
    clippy::mem_forget,
    clippy::unused_self,
    clippy::filter_map_next,
    clippy::needless_continue,
    clippy::needless_borrow,
    clippy::match_wildcard_for_single_variants,
    clippy::if_let_mutex,
    clippy::mismatched_target_os,
    clippy::await_holding_lock,
    clippy::match_on_vec_items,
    clippy::imprecise_flops,
    clippy::suboptimal_flops,
    clippy::lossy_float_literal,
    clippy::rest_pat_in_fully_bound_structs,
    clippy::fn_params_excessive_bools,
    clippy::exit,
    clippy::inefficient_to_string,
    clippy::linkedlist,
    clippy::macro_use_imports,
    clippy::option_option,
    clippy::verbose_file_reads,
    clippy::unnested_or_patterns,
    clippy::str_to_string,
    rust_2018_idioms,
    future_incompatible,
    nonstandard_style,
    missing_debug_implementations,
    missing_docs,
    rustdoc::missing_doc_code_examples
)]
#![deny(unreachable_pub, private_in_public)]
#![forbid(unsafe_code)]

mod key;
mod render;
mod traits;

pub mod engine;

pub use key::Key;
pub use render::{Render, RenderHtml};
pub use traits::TemplateEngine;
