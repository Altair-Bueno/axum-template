use std::sync::Arc;
use tower_http::add_extension::AddExtension;
use tower_layer::Layer;

#[cfg(feature = "handlebars")]
mod handlebars;
#[cfg(feature = "handlebars")]
pub use self::handlebars::*;

#[cfg(feature = "tera")]
mod tera;
#[cfg(feature = "tera")]
pub use self::tera::*;

#[cfg(feature = "tinytemplate")]
mod tinytemplate;
#[cfg(feature = "tinytemplate")]
pub use self::tinytemplate::*;

#[derive(Debug, Clone)]
pub struct Engine<E> {
    engine: Arc<E>,
}

impl<E> Engine<E> {
    pub fn new(engine: E) -> Self {
        let engine = Arc::new(engine);
        Self { engine }
    }
}

impl<E> From<E> for Engine<E> {
    fn from(engine: E) -> Self {
        Self::new(engine)
    }
}

impl<S, E> Layer<S> for Engine<E>
where
    E: Clone,
{
    type Service = AddExtension<S, Self>;

    fn layer(&self, inner: S) -> Self::Service {
        AddExtension::new(inner, self.clone())
    }
}
