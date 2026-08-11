mod core;

use crate::core::MorldCore;
use crate::core::types::MorldCoreRuntime;
use bevy::prelude::*;

fn print(
	core: Res<MorldCoreRuntime>
){
	for item in &core.key_final{
		println!("{:?} ", item);
	}
}

fn main() {
	App::new()
		.add_plugins(DefaultPlugins)
		.add_plugins(MorldCore)
		.add_systems(Update, print)
		.run();
}
