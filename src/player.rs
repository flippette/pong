use anyhow::Result;
use bevy::{math::*, prelude::*, sprite::*, window::*};
use bevy_rapier2d::prelude::*;

use crate::prelude::*;

const DEFAULT_PLAYER_SIZE: Vec2 = vec2(10., 40.);
const DEFAULT_PLAYER_SPEED: f32 = 1000.;

/// Plugin that handles player spawning, input and scoring
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Scores::default())
            .add_systems(
                Startup,
                (
                    spawn_players.pipe(log_error),
                    spawn_score_text.pipe(log_error),
                    init_last_window_size.pipe(log_error),
                ),
            )
            .add_systems(
                Update,
                (
                    handle_player_input,
                    move_players_with_window,
                    move_score_text_with_window,
                    update_score_text_on_scoring,
                    update_last_window_size
                        .after(move_players_with_window)
                        .after(move_score_text_with_window),
                ),
            );
    }
}

#[derive(Default, Debug, Resource)]
pub struct Scores {
    pub left: u32,
    pub right: u32,
}

#[derive(Debug, Resource)]
struct LastWindowSize {
    width: f32,
    height: f32,
}

fn spawn_players(
    mut commands: Commands,
    window: Query<&Window, With<PrimaryWindow>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) -> Result<()> {
    let window = window.get_single()?;

    let player_cuboid_size = DEFAULT_PLAYER_SIZE / 2.;
    let player_bundle = (
        // rendering components
        Mesh2dHandle(meshes.add(Cuboid::new(
            DEFAULT_PLAYER_SIZE.x,
            DEFAULT_PLAYER_SIZE.y,
            0.,
        ))),
        materials.add(ColorMaterial::from(Color::WHITE)),
        VisibilityBundle::default(),
        // physics components
        Collider::cuboid(player_cuboid_size.x, player_cuboid_size.y),
        Dominance::group(1),
        RigidBody::Dynamic,
        LockedAxes::ROTATION_LOCKED,
        GravityScale(0.),
        Restitution {
            coefficient: 1.,
            combine_rule: CoefficientCombineRule::Multiply,
        },
        Friction {
            coefficient: 0.,
            combine_rule: CoefficientCombineRule::Multiply,
        },
        Velocity::zero(),
        Speed(DEFAULT_PLAYER_SPEED),
    );

    commands.spawn((
        // "marker" components
        Player {
            up: KeyCode::KeyW,
            down: KeyCode::KeyS,
        },
        // transform
        TransformBundle::from_transform(Transform::from_xyz(
            -window.width() / 3.,
            0.,
            0.,
        )),
        player_bundle.clone(),
    ));

    commands.spawn((
        // "marker" components
        Player {
            up: KeyCode::KeyO,
            down: KeyCode::KeyL,
        },
        // transform
        TransformBundle::from_transform(Transform::from_xyz(
            window.width() / 3.,
            0.,
            0.,
        )),
        player_bundle,
    ));

    Ok(())
}

fn spawn_score_text(
    mut commands: Commands,
    window: Query<&Window, With<PrimaryWindow>>,
) -> Result<()> {
    let window = window.get_single()?;

    // left player score
    commands.spawn((
        Text2dBundle {
            text: Text::from_section(
                "0",
                TextStyle {
                    font_size: 30.,
                    ..default()
                },
            ),
            transform: Transform::from_xyz(
                -window.width() / 4.,
                window.height() * 3. / 8.,
                0.,
            ),
            ..default()
        },
        ScoreText::Left,
    ));

    // right player score
    commands.spawn((
        Text2dBundle {
            text: Text::from_section(
                "0",
                TextStyle {
                    font_size: 30.,
                    ..default()
                },
            ),
            transform: Transform::from_xyz(
                window.width() / 4.,
                window.height() * 3. / 8.,
                0.,
            ),
            ..default()
        },
        ScoreText::Right,
    ));

    Ok(())
}

/// Initialize the [`LastWindowSize`] resource.
fn init_last_window_size(
    mut commands: Commands,
    window: Query<&Window, With<PrimaryWindow>>,
) -> Result<()> {
    let window = window.get_single()?;
    let (width, height) = (window.width(), window.height());
    commands.insert_resource(LastWindowSize { width, height });
    Ok(())
}

/// Update the [`LastWindowSize`] resource per [`WindowResized`] event.
fn update_last_window_size(
    mut evr: EventReader<WindowResized>,
    mut lws: ResMut<LastWindowSize>,
) {
    for &WindowResized { width, height, .. } in evr.read() {
        *lws = LastWindowSize { width, height };
    }
}

/// Move players on [`WindowResized`].
fn move_players_with_window(
    mut players: Query<&mut Transform, With<Player>>,
    lws: Res<LastWindowSize>,
    mut evr: EventReader<WindowResized>,
) {
    for &WindowResized { width, height, .. } in evr.read() {
        for mut transform in &mut players {
            let mut new_translation2 = transform.translation.xy();
            new_translation2 /= vec2(lws.width, lws.height);
            new_translation2 *= vec2(width, height);
            transform.translation =
                vec3(new_translation2.x, new_translation2.y, 0.);
        }
    }
}

/// Move score texts on [`WindowResized`].
fn move_score_text_with_window(
    mut texts: Query<&mut Transform, With<ScoreText>>,
    lws: Res<LastWindowSize>,
    mut evr: EventReader<WindowResized>,
) {
    for &WindowResized { width, height, .. } in evr.read() {
        for mut transform in &mut texts {
            let mut new_translation2 = transform.translation.xy();
            new_translation2 /= vec2(lws.width, lws.height);
            new_translation2 *= vec2(width, height);
            transform.translation =
                vec3(new_translation2.x, new_translation2.y, 0.);
        }
    }
}

fn handle_player_input(
    mut players: Query<(&Player, &Speed, &mut Velocity)>,
    inputs: Res<ButtonInput<KeyCode>>,
) {
    for (player, speed, mut velocity) in &mut players {
        let modifier = i8::from(inputs.pressed(player.up))
            - i8::from(inputs.pressed(player.down));
        velocity.linvel.y = f32::from(modifier) * speed.0;
    }
}

fn update_score_text_on_scoring(
    mut texts: Query<(&mut Text, &ScoreText)>,
    scores: Res<Scores>,
) {
    if scores.is_changed() {
        for (mut text, score_text) in &mut texts {
            text.sections[0].value = match score_text {
                ScoreText::Left => scores.left,
                ScoreText::Right => scores.right,
            }
            .to_string();
        }
    }
}
