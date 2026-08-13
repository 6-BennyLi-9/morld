mod core;

use crate::core::MorldCore;
use crate::core::types::MorldCoreRuntime;
use bevy::prelude::*;
use bevy_fluent::Localization;
use fluent_content::Content;
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
	localization: Res<Localization>
) {
	match localization.content("debug") {
		None => {
			panic!("Error: Cannot find Object: debug")
		}
		Some(val) => {
			info!(val)
		}
	};
}