mod core;
pub mod menu;

use crate::core::MorldCore;
use crate::core::locales::{LocaleSettings, Localization};
use crate::core::process::game_states::{Errors, MorldStates};
use crate::menu::MorldMenu;
use bevy::prelude::*;
use crate::core::entity::{Carnal, Mage, Player, Entity};

fn main() {
	App::new()
		.add_plugins(DefaultPlugins)
		.add_plugins(MorldCore)
		.add_plugins(MorldMenu)
		.add_systems(OnEnter(MorldStates::MENU), |mut next_state: ResMut<NextState<MorldStates>>|{next_state.set(MorldStates::GAMING)})
		.add_systems(Update, print)
		.add_systems(Update, debug_exit)
		.add_systems(OnEnter(MorldStates::MENU), display_message)
		.run();
}

#[allow(dead_code)]
fn print(
	input: Res<ButtonInput<KeyCode>>,
	query: Query<(&Entity, &Carnal, &Mage, &Transform, &Player)>,
	query2: Query<&Carnal>,
){
	if input.just_pressed(KeyCode::F3) {
		if query.is_empty() {
			warn!("cannot found player!");
		} else {
			query.iter().for_each(|(entity, carnal, mage, _, _)| {
				info!("Player[{}]:[Health:{}/{}, Mana:{}/{}, 2Def:({:?},{:?})]",
					entity.id,
					carnal.health, carnal.health_maximum,
					mage.mana, mage.mana_maximum,
					carnal.amour, carnal.magic_resistance
				)
			})
		}

		query2.iter().for_each(|x| info!("{:?}", x));
	}
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

fn debug_exit(
	mut errors: ResMut<Errors>,
	input: Res<ButtonInput<KeyCode>>,
){
	if input.all_pressed([KeyCode::F11, KeyCode::F12]) {
		errors.errors.clear();
		errors.errors.push("DEBUG EXIT".to_owned());
	}
}
