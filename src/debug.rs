use anyhow::Result;
use bevy::prelude::*;

use crate::prelude::*;

/// Runtime assertions for debugging purposes.
pub struct AssertionsPlugin;

impl Plugin for AssertionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, ensure_one_main_camera.pipe(log_error));
    }
}

///
/// Ensures there is only ever one [`MainCamera`].
///
fn ensure_one_main_camera(query: Query<(), With<MainCamera>>) -> Result<()> {
    query.get_single()?;
    Ok(())
}
