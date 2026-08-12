mod core;

use std::io::SeekFrom::Start;
use std::thread::sleep;
use std::time::Duration;
use crate::core::MorldCore;
use bevy::prelude::*;
use bevy_fluent::{BundleAsset};
use crate::core::locales::{from_locale};
use crate::core::types::MorldCoreRuntime;

fn main() {
	App::new()
		.add_plugins(DefaultPlugins)
		.add_plugins(MorldCore)
		.add_systems(Update, display_message)
		// .add_plugins(DefaultPlugins)
		// .add_plugins(MorldCore)
		// // .add_systems(Update, print)
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
fn display_message(
	asset_server: Res<AssetServer>,
	assets: Res<Assets<BundleAsset>>,
	core: Res<MorldCoreRuntime>
) {
	if let Some(val) = from_locale("debug", &asset_server, &assets, &core) {
		info!("{}", val);
	} else {
		info!("NONE");
	}
}