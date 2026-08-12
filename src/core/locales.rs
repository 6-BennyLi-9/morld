use crate::core::types::MorldCoreRuntime;
use bevy::app::{App, Plugin, Startup, Update};
use bevy::asset::LoadState;
use bevy::log::info;
use bevy::prelude::{AssetServer, Assets, Bundle, Handle, Local, Res, ResMut};
use bevy_fluent::{BundleAsset, FluentPlugin, Locale};

pub struct FluentInitial;

fn code(locale_code: &str) -> Locale{
	Locale::new(locale_code.parse().expect("Failed to parse locale code"))
}

fn load_localization(
	asset_server: Res<AssetServer>,
	mut handle: Local<Option<Handle<BundleAsset>>>,
){
	let _ = handle.insert(asset_server.load("localization/zh-CN.ftl.yml"));
}

pub fn read_locale_raw(
	asset_server: Res<AssetServer>,
	assets: Res<Assets<BundleAsset>>,
	mut handle: Local<Option<Handle<BundleAsset>>>,
	locale_code: &str,
	read_id: &str,
) -> Option<String> {
	let handle = &*handle.get_or_insert_with(|| {
		asset_server.load(format!("localization/{}.ftl.yml", locale_code))
	});
	if let Some(LoadState::Loaded) = asset_server.get_load_state(handle) {
		let bundle = assets.get(handle).unwrap();
		let message = bundle.get_message(read_id).expect(format!("Message '{}' not found in localization", read_id).as_str());

		let pattern = message.value().expect("Message has no value");

		let mut errors = vec![];
		let value = bundle.format_pattern(pattern, None, &mut errors);
		Some(value.parse().unwrap())
	} else { None }
}

impl Plugin for FluentInitial {
	fn build(&self, app: &mut App) {
        app
			.add_plugins(FluentPlugin)
			.insert_resource(code("zh-CN"))
			.add_systems(Update, load_localization);
	}
}
