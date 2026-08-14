mod core;
pub mod menu;

use crate::core::MorldCore;
use crate::core::locales::{LocaleSettings, Localization};
use crate::core::process::game_states::{Errors, MorldStates};
use crate::core::process::tasks::MorldTasks;
use crate::menu::MorldMenu;
use bevy::prelude::*;

fn main() {
	App::new()
		.add_plugins(DefaultPlugins)
		.add_plugins(MorldCore)
		.add_plugins(MorldMenu)
		.add_systems(Update, print.run_if(in_state(MorldStates::INITIALIZING)))
		.add_systems(OnEnter(MorldStates::MENU), display_message)
		.add_systems(OnEnter(MorldStates::MENU), errors)
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
	mut errors: ResMut<Errors>,
) {
	match localization.content(String::from("debug"), locale_settings) {
		Err(_) => {
			errors.errors.push("Cannot load locale item".to_owned());
		}
		Ok(val) => {
			info!(val)
		}
	};
}

#[allow(dead_code)]
fn errors(
	mut errors: ResMut<Errors>,
) {
	errors.errors.push("Cannot load locale item".to_owned());
	errors.errors.push("Cannot load locale item".to_owned());
	errors.errors.push("Cannot load locale item".to_owned());
	errors.errors.push("Cannot load locale item".to_owned());
}