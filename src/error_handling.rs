use std::fmt::Debug;
use bevy::prelude::In;

pub fn panic_on_error<TResult, TError: Debug>(In(result): In<Result<TResult, TError>>) {
    if let Err(err) = result {
        panic!("System has failed: {:?}", err);
    }
}
