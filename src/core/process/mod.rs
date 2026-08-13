use bevy::app::App;
use bevy::log::error;
use bevy::prelude::{in_state, AppExtStates, IntoScheduleConfigs, NextState, Plugin, Res, ResMut, Update, MessageWriter};
use crate::core::audio::{PlaySound, SoundType};
use crate::core::process::game_states::{Errors, MorldStates};
use crate::core::process::tasks::MorldTasks;

pub mod tasks;
pub mod game_states;

pub struct MorldProcessPlugin;

fn on_finish_initializing(
	mut next_state: ResMut<NextState<MorldStates>>,
	tasks: Res<MorldTasks>
){
	if tasks.init.is_empty() {
		next_state.set(MorldStates::MENU);
	}
}

fn on_error(
	mut next_state: ResMut<NextState<MorldStates>>,
	mut writer: MessageWriter<PlaySound>,
	errors: Res<Errors>,
){
	if !errors.errors.is_empty() {
		for error in &errors.errors {
			error!("on_error received! message = {}", error);
		}
		
		writer.write(PlaySound{
			sound: SoundType::Ohno,
			volume: 1.0,
		});
		
		next_state.set(MorldStates::ERR);
	}
}

impl Plugin for MorldProcessPlugin {
	fn build(&self, app: &mut App) {
		app
			.init_state::<MorldStates>()
			.init_resource::<MorldTasks>()
			.init_resource::<Errors>()
			.add_systems(Update, on_finish_initializing.run_if(in_state(MorldStates::INITIALIZING)))
			.add_systems(Update, on_error);
	}
}
