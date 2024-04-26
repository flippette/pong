use core::fmt::Debug;

use bevy::prelude::*;

///
/// Log errors to the console.
///
/// Used with systems returning [`Result`] via [`IntoSystem::pipe`].
///
pub fn log_error<T, E: Debug>(In(res): In<Result<T, E>>) {
    if let Err(err) = res {
        error!("encountered error: {err:?}");
    }
}
