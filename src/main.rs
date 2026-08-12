mod core;

use bevy::asset::LoadState;
use crate::core::MorldCore;
use bevy::prelude::*;
use bevy_fluent::{BundleAsset, Localization};
use crate::core::locales::from_locale_raw;
use crate::core::types::MorldCoreRuntime;

fn main() {
	App::new()
		.add_plugins(DefaultPlugins)
		.add_plugins(MorldCore)
		// .add_systems(Update, print)
		.add_systems(Update, display_message)
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
	mut handle: Local<Option<Handle<BundleAsset>>>,
) {
	if let Some(val) = from_locale_raw(&asset_server, &assets, &mut handle, "zh-CN", "debug") {
		info!("{}", val);
	}
}