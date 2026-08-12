mod core;

use crate::core::MorldCore;
use crate::core::types::MorldCoreRuntime;
use bevy::prelude::*;

fn main() {
	App::new()
		.add_plugins(DefaultPlugins)
		.add_plugins(MorldCore)
		// .add_systems(Update, print)
		// .add_systems(Update, display_message)
		.run();
}

#[allow(dead_code)]
pub fn print(
	core: Res<MorldCoreRuntime>,
	time: Res<Time>,
){
	if let Ok(val) = core.key_input_time(KeyCode::Space, time.elapsed_secs_f64()) {
		println!("{}", val);
	}
}
#[allow(dead_code)]
fn display_message(
) {
}