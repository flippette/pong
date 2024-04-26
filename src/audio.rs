use bevy::prelude::*;

/// Plugin that handles loading & playing sound effects.
pub struct SoundEffectPlugin;

impl Plugin for SoundEffectPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SfxRequestEvent>()
            .add_systems(Startup, load_sfx)
            .add_systems(Update, play_sound_on_request);
    }
}

#[derive(Debug, Resource)]
struct SoundEffects {
    bounce: Handle<AudioSource>,
    score: Handle<AudioSource>,
}

#[derive(Debug, Event)]
pub enum SfxRequestEvent {
    Bounce,
    Score,
}

fn load_sfx(mut commands: Commands, assets: Res<AssetServer>) {
    commands.insert_resource(SoundEffects {
        bounce: assets.load("sfx/bounce.wav"),
        score: assets.load("sfx/score.wav"),
    });
}

fn play_sound_on_request(
    mut commands: Commands,
    sfx: Res<SoundEffects>,
    mut evr: EventReader<SfxRequestEvent>,
) {
    for ev in evr.read() {
        commands.spawn(AudioSourceBundle {
            source: match ev {
                SfxRequestEvent::Bounce => sfx.bounce.clone(),
                SfxRequestEvent::Score => sfx.score.clone(),
            },
            ..default()
        });
    }
}
