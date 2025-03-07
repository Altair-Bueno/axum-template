# axum-template

[![LICENSE](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Test status](https://github.com/Altair-Bueno/axum-template/actions/workflows/test.yml/badge.svg)](https://github.com/Altair-Bueno/axum-template/actions/workflows/test.yml)
[![Crates.io Version](https://img.shields.io/crates/v/axum-template.svg)](https://crates.io/crates/axum-template)
[![docs.rs](https://img.shields.io/docsrs/axum-template)](https://docs.rs/axum-template)
![MSRV](https://img.shields.io/crates/msrv/axum-template)

Layers, extractors and template engine wrappers for [axum] based Web MVC
applications

# Getting started

The [`engine`] module contains detailed usage examples for each of the supported
template engines.

If you plan using an unsupported engine, check the [`TemplateEngine`] docs

# Available features

- `handlebars`: Enables [handlebars] support
- `minijinja`: Enables [minijinja] support
- `minijinja-autoreload`: Enables [minijinja-autoreload] support
- `tera`: Enables [tera] support

# Useful links

- [Documentation]
- [Examples]
- [Source code]

## Learning resources

Tutorials, blog posts and success stories not affiliated to this project. They
might be useful for new commers of the Rust programming language or experienced
devs that would like to see this library in action.

- [Server-side rendering in Rust - a Dall.E use-case](https://blog.frankel.ch/server-side-rendering-rust/)

# License

Licensed under the MIT license. See [LICENSE] for more information

[`engine`]: crate::engine
[`TemplateEngine`]: crate::TemplateEngine
[LICENSE]: https://github.com/Altair-Bueno/axum-template/blob/main/LICENSE
[Documentation]: https://docs.rs/axum-template
[Examples]: https://github.com/Altair-Bueno/axum-template/tree/main/examples
[Source code]: https://github.com/Altair-Bueno/axum-template
[axum]: https://github.com/tokio-rs/axum
[handlebars]: https://crates.io/crates/handlebars
[minijinja]: https://crates.io/crates/minijinja
[minijinja-autoreload]: https://crates.io/crates/minijinja-autoreload
[tera]: https://crates.io/crates/tera
