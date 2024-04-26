use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::prelude::*;

/// Plugin that handles physics interactions not covered by Rapier
/// (e.g. speed limiting)
pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, align_velocity_to_speed);
    }
}

/// Force magnitude of velocity to be equal to speed
fn align_velocity_to_speed(mut query: Query<(&Speed, &mut Velocity)>) {
    for (speed, mut velocity) in &mut query {
        velocity.linvel = velocity.linvel.normalize_or_zero() * speed.0;
    }
}
