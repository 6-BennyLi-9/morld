mod core;

use crate::core::MorldCore;
use bevy::prelude::*;
use crate::core::locales::{LocaleSettings, Localization};
use crate::core::process::game_states::MorldStates;

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
){
	info!("IN STATE: INITIALIZING")
}
#[allow(dead_code)]
fn display_message(
	localization: Res<Localization>,
	locale_settings: Res<LocaleSettings>,
) {
	match localization.content(String::from("debug"), locale_settings) {
		Err(_) => {
			panic!("Error: Cannot find Object: debug")
		}
		Ok(val) => {
			info!(val)
		}
	};
}