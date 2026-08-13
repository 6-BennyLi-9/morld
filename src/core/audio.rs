use crate::core::process::tasks::InitialTasks::LoadAudio;
use crate::core::process::tasks::MorldTasks;
use bevy::audio::Volume;
use bevy::ecs::message::MessageReader;
use bevy::prelude::*;

///LOAD ALL AUDIOS
#[derive(Resource)]
pub struct AudioAssets {
	pub ohno: Handle<AudioSource>,
}


/// BGM
#[derive(Component)]
pub struct BackgroundMusic;


/// 音效类型
#[derive(bevy::ecs::message::Message, Clone)]
pub enum SoundType {
	Ohno,
}

#[derive(bevy::ecs::message::Message, Clone)]
pub struct PlaySound {
	pub sound: SoundType,
	pub volume: f32,
}

pub fn load_audio_assets(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	mut tasks: ResMut<MorldTasks>,
) {
	commands.insert_resource(AudioAssets {
		ohno: asset_server.load("audio/ohno.ogg"),
	});

	tasks.init.remove(&LoadAudio);
}

pub fn play_sound_effects(
	mut messages: MessageReader<PlaySound>,
	mut commands: Commands,
	audio_assets: Res<AudioAssets>,
) {
	for event in messages.read() {
		let handle = match event.sound {
			SoundType::Ohno => audio_assets.ohno.clone(),
		};
		commands.spawn((
			AudioPlayer::new(handle),
			PlaybackSettings::DESPAWN.with_volume(Volume::Linear(event.volume)),
		));
	}
}

pub struct AudioPlugin;
impl Plugin for AudioPlugin {
	fn build(&self, app: &mut App) {
		app
			.add_message::<PlaySound>()
			.add_systems(Startup, load_audio_assets)
			.add_systems(Update, play_sound_effects);
	}
}