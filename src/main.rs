mod core;

use crate::core::MorldCore;
use bevy::prelude::*;
use crate::core::audio::{PlaySound, SoundType};
use crate::core::locales::{LocaleSettings, Localization};
use crate::core::process::game_states::MorldStates;
use crate::core::process::tasks::MorldTasks;

fn main() {
	App::new()
		.add_plugins(DefaultPlugins)
		.add_plugins(MorldCore)
		.add_systems(Update, print.run_if(in_state(MorldStates::INITIALIZING)))
		.add_systems(OnEnter(MorldStates::MENU), display_message)
		.run();
}

#[allow(dead_code)]
pub fn print(
	tasks: Res<MorldTasks>
){
	info!("INITIALIZING for {:?}", tasks.init);
}
#[allow(dead_code)]
fn display_message(
	localization: Res<Localization>,
	locale_settings: Res<LocaleSettings>,
	mut error: MessageWriter<PlaySound>
) {
	match localization.content(String::from("debug"), locale_settings) {
		Err(_) => {
			error.write(PlaySound{
				sound: SoundType::Ohno,
				volume: 1.0,
			});
		}
		Ok(val) => {
			info!(val)
		}
	};
}