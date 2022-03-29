#![feature(if_let_guard)]
#![warn(clippy::pedantic)]
#![allow(clippy::default_trait_access)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::must_use_candidate)]
#![allow(clippy::needless_pass_by_value)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::type_complexity)]

mod error_handling;
mod math;
mod vec_extensions;

pub use error_handling::*;
pub use math::*;
pub use vec_extensions::*;
