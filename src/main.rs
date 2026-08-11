mod core;

use bevy::prelude::*;
use bevy::sprite_render::Wireframe2dPlugin;

fn main() {
	App::new()
		.add_plugins(DefaultPlugins)
		.add_plugins(#[cfg(not(target_arch = "wasm32"))]Wireframe2dPlugin::default())
		.run();
}
