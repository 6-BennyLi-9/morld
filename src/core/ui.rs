use crate::core::process::tasks::InitialTasks::CameraSetup;
use crate::core::process::tasks::MorldTasks;
use bevy::app::{App, Plugin, Startup};
use bevy::camera::Camera2d;
use bevy::prelude::{Commands, Query, ResMut, Window};

pub struct UIPlugin;

fn camera_setup(
	mut commands: Commands,
	mut tasks: ResMut<MorldTasks>,
) {
	commands.spawn(Camera2d);

	tasks.init.remove(&CameraSetup);
}
fn joking_window_name(
	mut query: Query<&mut Window>
) {
	let names: Vec<&str> = vec![
		"Morld: YOU'RE TAKING TOO LONG                    *for rustc*",
		"Morld: who wants to be a million-crab-aire",
		"Morld:World",
		"World:Morld",
		"Morld: 船新版本，震撼来袭",
	];

	query.single_mut().unwrap().title = names[(rand::random::<u8>() % names.len() as u8) as usize].to_string();
}
impl Plugin for UIPlugin {
	fn build(&self, app: &mut App) {
		app
			.add_systems(Startup, camera_setup)
			.add_systems(Startup, joking_window_name);
	}
}
