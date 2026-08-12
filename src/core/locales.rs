use crate::core::types::MorldCoreRuntime;
use bevy::app::{App, Plugin, Startup, Update};
use bevy::asset::LoadState;
use bevy::prelude::{AssetServer, Assets, Bundle, Handle, Local, Res, ResMut};
use bevy_fluent::{BundleAsset, FluentPlugin, Locale};

pub struct FluentInitial;

fn code(locale_code: &str) -> Locale{
	Locale::new(locale_code.parse().expect("Failed to parse locale code"))
}

fn load_localization(
	asset_server: Res<AssetServer>,
	assets: Res<Assets<BundleAsset>>,
	mut handle: Local<Option<Handle<BundleAsset>>>,
){
	let handle = &*handle.get_or_insert_with(|| asset_server.load("localization/zh-CN.ftl.yml"));
	if let Some(LoadState::Loaded) = asset_server.get_load_state(handle) {
		let bundle = assets.get(handle).unwrap();
		let message = bundle.get_message("debug").expect("Message 'debug' not found in localization");
		println!("Loaded message '{:?}'", message.value().unwrap().elements[0]);
	}
}

impl Plugin for FluentInitial {
	fn build(&self, app: &mut App) {
        app
			.add_plugins(FluentPlugin)
			.insert_resource(code("zh-CN"))
			.add_systems(Update, load_localization);
	}
}
