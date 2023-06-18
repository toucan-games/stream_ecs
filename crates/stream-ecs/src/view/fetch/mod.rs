pub use self::impls::{FetchComponent, FetchEntity, FetchOption};

mod impls;

pub trait Fetch<'a>: 'a {
    type Item: 'a;
}
