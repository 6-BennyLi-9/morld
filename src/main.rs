mod core;

use crate::core::MorldCore;
use bevy::prelude::*;
use crate::core::types::MorldCoreRuntime;

fn main() {
	App::new()
		.add_plugins(DefaultPlugins)
		.add_plugins(MorldCore)
		.add_systems(Update, print)
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