mod core;

use crate::core::MorldCore;
use bevy::prelude::*;

fn main() {
	App::new()
		.add_plugins(DefaultPlugins)
		.add_plugins(MorldCore)
		.add_systems(Update, morld::test::print)
		.run();
}
