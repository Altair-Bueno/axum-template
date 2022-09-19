# axum-template
<!-- cargo-sync-readme start -->

Layers, extractors and template engine wrappers for
[axum] based Web MVC applications

# Getting started

## `Cargo.toml`

```toml
[dependencies]
axum-template = "*"
```

The [`engine`] module contains detailed usage examples for each of the
supported template engines.

If you plan using an unsupported engine, check the [`TemplateEngine`] docs

# Available features

- `handlebars`: Enables [handlebars] support
- `minijinja`: Enables [minijinja] support
- `tera`: Enables [tera] support

# Useful links

- [Documentation]
- [Examples]
- [Source code]

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
[tera]: https://crates.io/crates/tera


<!-- cargo-sync-readme end -->