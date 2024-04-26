use bevy::prelude::*;

///
/// The camera that renders the menus and the playfield.
///
/// Systems querying for this camera should use [`Query::get_single`],
/// panicking in this case should _NOT_ happen.
///
#[derive(Debug, Component)]
pub struct MainCamera;

/// The top & bottom bounds of the playfield.
#[derive(Clone, Copy, Debug, Component)]
pub enum Bound {
    Top,
    Bottom,
}

/// The left & right goals.
#[derive(Clone, Copy, Debug, Component)]
pub enum Goal {
    Left,
    Right,
}

/// The pong ball.
#[derive(Clone, Copy, Debug, Component)]
pub struct Ball;

/// The player.
#[derive(Clone, Copy, Debug, Component)]
pub struct Player {
    pub up: KeyCode,
    pub down: KeyCode,
}

#[derive(Clone, Copy, Debug, Component)]
pub struct Speed(pub f32);

#[derive(Debug, Component)]
pub enum ScoreText {
    Left,
    Right,
}
