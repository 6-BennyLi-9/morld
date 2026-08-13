use bevy::app::{App, Plugin, Startup};
use bevy::camera::Camera2d;
use bevy::prelude::{Commands, Res, ResMut};
use sys_locale::get_locale;
use crate::core::locales::{load_locales, LocaleSettings, Localization};
use crate::core::process::tasks::InitialTasks::CameraSetup;
use crate::core::process::tasks::MorldTasks;

pub struct UIPlugin;

fn camera_setup(
	mut commands: Commands,
	mut tasks: ResMut<MorldTasks>
) {
	commands.spawn(Camera2d);
	
	tasks.init.remove(&CameraSetup);
}

impl Plugin for UIPlugin {
	fn build(&self, app: &mut App) {
		app
			.add_systems(Startup, camera_setup);
	}
}
