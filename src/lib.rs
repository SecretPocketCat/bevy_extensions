#![feature(destructuring_assignment)]
#![feature(if_let_guard)]

#![warn(clippy::pedantic)]
#![allow(clippy::default_trait_access)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::must_use_candidate)]
#![allow(clippy::needless_pass_by_value)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::type_complexity)]

mod vec_extensions;
mod error_handling;

pub use vec_extensions::{Vec2Conversion, Vec2Extensions, VecAxis};
pub use error_handling::panic_on_error;
