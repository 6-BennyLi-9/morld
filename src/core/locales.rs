use bevy::app::{App, Plugin, Update};
use bevy::asset::LoadState;
use bevy::prelude::{AssetServer, Assets, Handle, Local, Res};
use bevy_fluent::{BundleAsset, FluentPlugin, Locale};
use crate::core::types::MorldCoreRuntime;

pub struct FluentInitial;

fn code(locale_code: &str) -> Locale {
	Locale::new(locale_code.parse().expect("Failed to parse locale code"))
}

fn load_localization(
	asset_server: Res<AssetServer>,
	mut handle: Local<Option<Handle<BundleAsset>>>,
){
	let _ = handle.insert(asset_server.load("localization/zh-CN.ftl.yml"));
}

pub fn from_locale_raw(
	asset_server: &Res<AssetServer>,
	assets: &Res<Assets<BundleAsset>>,
	locale_code: &str,
	read_id: &str,
) -> Option<String> {
	let handle :Handle<BundleAsset> = asset_server.load(format!("localization/{}.ftl.yml", locale_code));
	if let Some(LoadState::Loaded) = asset_server.get_load_state(&handle) {
		let bundle = assets.get(&handle).unwrap();
		let message = bundle.get_message(read_id).expect(format!("Message '{}' not found in localization", read_id).as_str());

		let pattern = message.value().expect("Message has no value");

		let mut errors = vec![];
		let value = bundle.format_pattern(pattern, None, &mut errors);
		Some(value.parse().unwrap())
	} else { None }
}

pub fn from_locale(
	read_id: &str,
	asset_server: &Res<AssetServer>,
	assets: &Res<Assets<BundleAsset>>,
	core: &Res<MorldCoreRuntime>,
) -> Option<String> {
	let temp = from_locale_raw(asset_server, assets, core.current_lang.as_str(), read_id);
	if temp.is_some() {
		temp
	} else {
		from_locale_raw(asset_server, assets, core.default_lang.as_str(), read_id)
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
