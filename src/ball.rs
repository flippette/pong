use bevy::{math::*, prelude::*, sprite::*};
use bevy_rapier2d::{
    pipeline::*, prelude::*, rapier::geometry::CollisionEventFlags,
};
use rand::{thread_rng, Rng};

use crate::prelude::*;

const DEFAULT_BALL_RADIUS: f32 = 5.;
const DEFAULT_BALL_SPEED: f32 = 300.;

/// Plugin that handle all ball-related functionality.
pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_ball)
            .add_systems(Update, (handle_scoring, play_sfx_on_contact));
    }
}

/// Generate a random, normalized direction vector that points at
/// 45-degree corners.
fn random_direction() -> Vec2 {
    match thread_rng().gen_range(0..4) {
        0 => vec2(-1., 1.),
        1 => vec2(1., 1.),
        2 => vec2(1., -1.),
        3 => vec2(-1., -1.),
        _ => unreachable!(),
    }
    .normalize()
}

fn spawn_ball(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        // marker components
        Ball,
        // rendering components
        ColorMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Circle::new(DEFAULT_BALL_RADIUS))),
            material: materials.add(ColorMaterial::from(Color::WHITE)),
            ..default()
        },
        // physics components
        Collider::ball(DEFAULT_BALL_RADIUS),
        Dominance::group(i8::MIN),
        RigidBody::Dynamic,
        ActiveEvents::COLLISION_EVENTS,
        LockedAxes::ROTATION_LOCKED,
        Ccd::enabled(),
        GravityScale(0.),
        Restitution {
            coefficient: 1.,
            combine_rule: CoefficientCombineRule::Multiply,
        },
        Velocity::linear(random_direction() * DEFAULT_BALL_SPEED),
        Speed(DEFAULT_BALL_SPEED),
    ));
}

/// Respawn the ball & update the score text
fn handle_scoring(
    mut balls: Query<
        (Entity, &Speed, &mut Velocity, &mut Transform),
        (With<Ball>, With<Collider>),
    >,
    goals: Query<(Entity, &Goal), With<Collider>>,
    mut collision_evr: EventReader<CollisionEvent>,
    mut scores: ResMut<Scores>,
) {
    for ev in collision_evr.read() {
        if let &CollisionEvent::Started(
            sensor,
            foreign,
            CollisionEventFlags::SENSOR,
        ) = ev
        {
            for (ball_entity, speed, mut velocity, mut transform) in &mut balls
            {
                for (goal_entity, goal) in &goals {
                    // the sensor comes first
                    if sensor == goal_entity && foreign == ball_entity {
                        // scored!

                        // bump score
                        match goal {
                            Goal::Left => scores.right += 1,
                            Goal::Right => scores.left += 1,
                        }

                        // "respawn" ball
                        transform.translation = Vec3::ZERO;
                        velocity.linvel = random_direction() * speed.0;
                    }
                }
            }
        }
    }
}

fn play_sfx_on_contact(
    mut evr: EventReader<CollisionEvent>,
    mut evw: EventWriter<SfxRequestEvent>,
    goals: Query<Entity, With<Goal>>,
) {
    for ev in evr.read() {
        if let &CollisionEvent::Started(a, b, _) = ev {
            evw.send(
                if goals.iter().any(|entity| entity == a || entity == b) {
                    SfxRequestEvent::Score
                } else {
                    SfxRequestEvent::Bounce
                },
            );
        }
    }
}
