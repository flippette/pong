use anyhow::Result;
use bevy::{prelude::*, window::*};
use bevy_rapier2d::prelude::*;

use crate::prelude::*;

/// Plugin that handles spawning and resizing the field.
pub struct FieldPlugin;

impl Plugin for FieldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            (spawn_bounds.pipe(log_error), spawn_goals.pipe(log_error)),
        )
        .add_systems(
            Update,
            (resize_bounds_with_window, resize_goals_with_window),
        );
    }
}

fn spawn_bounds(
    mut commands: Commands,
    window: Query<&Window, With<PrimaryWindow>>,
) -> Result<()> {
    let window = window.get_single()?;

    let bound_bundle = (
        // physics components
        Collider::cuboid(window.width() / 2., 0.5),
        Dominance::group(i8::MAX),
        RigidBody::Fixed,
        Restitution {
            coefficient: 1.,
            combine_rule: CoefficientCombineRule::Multiply,
        },
        Friction {
            coefficient: 0.,
            combine_rule: CoefficientCombineRule::Multiply,
        },
    );
    commands.spawn((
        // marker components
        Bound::Top,
        // transform
        TransformBundle::from_transform(Transform::from_xyz(
            0.,
            window.height() / 2.,
            0.,
        )),
        // the bundle
        bound_bundle.clone(),
    ));
    commands.spawn((
        // marker components
        Bound::Bottom,
        // transform
        TransformBundle::from_transform(Transform::from_xyz(
            0.,
            -window.height() / 2.,
            0.,
        )),
        // the bundle
        bound_bundle,
    ));

    Ok(())
}

fn spawn_goals(
    mut commands: Commands,
    window: Query<&Window, With<PrimaryWindow>>,
) -> Result<()> {
    let window = window.get_single()?;

    let goal_bundle = (
        Collider::cuboid(0.5, window.height() / 2.),
        Sensor,
        ActiveEvents::COLLISION_EVENTS,
    );
    commands.spawn((
        // marker components
        Goal::Left,
        // transform
        TransformBundle::from_transform(Transform::from_xyz(
            -window.width() / 2.,
            0.,
            0.,
        )),
        // the bundle
        goal_bundle.clone(),
    ));
    commands.spawn((
        // marker components
        Goal::Right,
        // transform
        TransformBundle::from_transform(Transform::from_xyz(
            window.width() / 2.,
            0.,
            0.,
        )),
        // the bundle
        goal_bundle,
    ));

    Ok(())
}

fn resize_bounds_with_window(
    mut bounds: Query<(&Bound, &mut Transform, &mut Collider)>,
    mut evr: EventReader<WindowResized>,
) {
    for WindowResized { width, height, .. } in evr.read() {
        for (bound, mut transform, mut collider) in &mut bounds {
            transform.translation.y = match bound {
                Bound::Top => height / 2.,
                Bound::Bottom => -height / 2.,
            };

            *collider = Collider::cuboid(width / 2., 0.5);
        }
    }
}

fn resize_goals_with_window(
    mut goals: Query<(&Goal, &mut Transform, &mut Collider)>,
    mut evr: EventReader<WindowResized>,
) {
    for WindowResized { width, height, .. } in evr.read() {
        for (goal, mut transform, mut collider) in &mut goals {
            transform.translation.x = match goal {
                Goal::Left => -width / 2.,
                Goal::Right => width / 2.,
            };

            *collider = Collider::cuboid(0.5, height / 2.);
        }
    }
}
