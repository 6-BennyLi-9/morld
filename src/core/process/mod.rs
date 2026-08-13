use bevy::app::App;
use bevy::prelude::{in_state, AppExtStates, Commands, IntoScheduleConfigs, NextState, Plugin, Res, ResMut, Update};
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

impl Plugin for MorldProcessPlugin {
	fn build(&self, app: &mut App) {
		app
			.init_state::<MorldStates>()
			.init_resource::<MorldTasks>()
			.init_resource::<Errors>()
			.add_systems(Update, on_finish_initializing.run_if(in_state(MorldStates::INITIALIZING)));
	}
}
