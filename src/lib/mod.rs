mod arithmetic;
mod grid;
pub mod solution;

pub mod prelude {
    pub use super::arithmetic::*;
    pub use super::grid::*;
    pub use proc_macros::*;
}

#[macro_export]
macro_rules! impl_for_all {
    ([$($t:ty),*], impl $trait:ident $impl:tt) => {
        $(
            impl $trait for $t $impl
        )*
    };
}
