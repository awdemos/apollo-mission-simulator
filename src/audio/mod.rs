use bevy::prelude::*;

pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AudioAssets>()
            .add_systems(Startup, load_audio_assets)
            .add_systems(Update, play_dsky_clicks.run_if(in_state(crate::game_state::AppState::InGame)));
    }
}

#[derive(Resource, Default)]
pub struct AudioAssets {
    pub dsky_click: Handle<AudioSource>,
}

fn load_audio_assets(mut assets: ResMut<AudioAssets>, asset_server: Res<AssetServer>) {
    assets.dsky_click = asset_server.load("sounds/dsky_click.wav");
}

fn play_dsky_clicks(
    mut interaction_events: EventReader<crate::panels::PanelInteraction>,
    audio_assets: Res<AudioAssets>,
    mut commands: Commands,
) {
    for event in interaction_events.read() {
        if let crate::panels::PanelInteraction::KeyPressed(_, _) = event {
            commands.spawn(AudioBundle {
                source: audio_assets.dsky_click.clone(),
                settings: PlaybackSettings::DESPAWN.with_volume(bevy::audio::Volume::new(1.0)),
                ..default()
            });
        }
    }
}
