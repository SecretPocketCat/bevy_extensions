#![feature(destructuring_assignment)]
#![feature(if_let_guard)]

mod vec_extensions;
mod error_handling;

pub use vec_extensions::{Vec2Conversion, Vec2Extensions, VecAxis};
pub use error_handling::panic_on_error;
