// Hide the command prompt in Windows release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(clippy::wildcard_imports)]

use bevy::{prelude::*, window::*};
use bevy_rapier2d::prelude::*;
use pong::prelude::*;

fn main() {
    let mut app = App::new();

    // default plugins
    app.add_plugins(
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    mode: WindowMode::Windowed,
                    present_mode: PresentMode::AutoNoVsync,
                    resolution: (800., 600.).into(),
                    title: env!("CARGO_PKG_NAME").into(),
                    ..default()
                }),
                ..default()
            })
            .set(AssetPlugin {
                file_path: "assets".to_string(),
                processed_file_path: "assets".to_string(),
                ..default()
            }),
    );

    // rapier physics plugin
    app.add_plugins(RapierPhysicsPlugin::<NoUserData>::default());

    // debug plugins
    #[cfg(debug_assertions)]
    app.add_plugins((AssertionsPlugin, RapierDebugRenderPlugin::default()));

    // gameplay plugins
    app.add_plugins((
        FieldPlugin,
        BallPlugin,
        PlayerPlugin,
        PhysicsPlugin,
        SoundEffectPlugin,
    ));

    // the setup system
    app.add_systems(Startup, setup);

    app.run();
}

fn setup(mut commands: Commands) {
    // spawn the main camera
    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                clear_color: ClearColorConfig::Custom(Color::BLACK),
                ..default()
            },
            ..default()
        },
        MainCamera,
    ));
}
