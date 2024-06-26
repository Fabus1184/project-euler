mod arithmetic;
mod grid;
mod primes;

pub mod prelude {
    pub use super::arithmetic::*;
    pub use super::grid::*;
    pub use super::primes::*;
}

#[macro_export]
macro_rules! impl_for_all {
    ([$($t:ty),*], impl $trait:ident $impl:tt) => {
        $(
            impl $trait for $t $impl
        )*
    };
}
