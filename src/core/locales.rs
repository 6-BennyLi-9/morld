use crate::core::types::MorldCoreRuntime;
use bevy::app::{App, Plugin, Startup, Update};
use bevy::asset::{Assets, LoadState};
use bevy::prelude::{AssetServer, Handle, Message, MessageReader, MessageWriter, Res, ResMut, Resource};
use bevy_fluent::{BundleAsset, FluentPlugin, Locale};
use std::collections::HashMap;

#[derive(Resource, Default)]
pub struct MorldLang{
	pub lang_handlers: HashMap<String, Handle<BundleAsset>>,
	pub lang_bundles: HashMap<String, BundleAsset>,
}
pub struct FluentInitial;

fn code(locale_code: &str) -> Locale {
	Locale::new(locale_code.parse().expect("Failed to parse locale code"))
}
#[derive(Message, Default, Clone)]
pub struct BundleOpening{
	pub lang: String,
	pub handler:Handle<BundleAsset>,
}

#[derive(Message, Default, Clone)]
pub struct BundleBlocking{
	pub lang: String,
	pub handler:Handle<BundleAsset>,
}

impl BundleOpening{
	pub fn to_blocking(&self) -> BundleBlocking {
		BundleBlocking{
			lang: self.lang.clone(),
			handler: self.handler.clone(),
		}
	}
}

impl BundleBlocking{
	pub fn to_opening(&self) -> BundleOpening {
		BundleOpening{
			lang: self.lang.clone(),
			handler: self.handler.clone(),
		}
	}
}

pub fn lang_init(
	core: Res<MorldCoreRuntime>,
	mut lang: ResMut<MorldLang>,
	asset_server: ResMut<AssetServer>,
	mut sender: MessageWriter<BundleOpening>
){
	let handle = asset_server.load(format!("localization/{}.ftl.yml", core.default_lang.clone()));
	lang.lang_handlers.insert(core.default_lang.clone(), handle.clone());
	sender.write(BundleOpening{
		handler: handle,
		lang: core.default_lang.clone(),
	});
	let handle = asset_server.load(format!("localization/{}.ftl.yml", core.current_lang.clone()));
	lang.lang_handlers.insert(core.current_lang.clone(), handle.clone());
	sender.write(BundleOpening{
		handler: handle,
		lang: core.current_lang.clone(),
	});
}

pub fn bundle_open_refresh(
	mut targets: MessageReader<BundleOpening>,
	mut incomplete: MessageWriter<BundleBlocking>,
	asset_server: Res<AssetServer>,
	assets: Res<Assets<BundleAsset>>,
	mut lang: ResMut<MorldLang>,
){
	for bundle in targets.read() {
		if let Some(LoadState::Loaded) = asset_server.get_load_state(&bundle.handler) {
			lang.lang_bundles.insert(bundle.lang.clone(), assets.get(&bundle.handler).unwrap().clone());
		} else {
			incomplete.write(bundle.to_blocking());
		}
	}
}

pub fn blocking_to_opening(
	mut from: MessageReader<BundleBlocking>,
	mut to: MessageWriter<BundleOpening>,
){
	for bundle in from.read() {
		to.write(bundle.to_opening());
	}
}

pub fn from_locale_raw(
	lang: &Res<MorldLang>,
	locale_code: &str,
	read_id: &str,
) -> Option<String> {
	if lang.lang_bundles.contains_key(&locale_code.to_string()) {
		let bundle = &lang.lang_bundles[&locale_code.to_string()];
		let message = bundle.get_message(read_id).expect(format!("Message '{}' not found in localization", read_id).as_str());

		let pattern = message.value().expect("Message has no value");

		let mut errors = vec![];
		let value = bundle.format_pattern(pattern, None, &mut errors);
		Some(value.parse().unwrap())
	} else { None }
}

pub fn from_locale(
	read_id: &str,
	lang: &Res<MorldLang>,
	core: &Res<MorldCoreRuntime>,
) -> Option<String> {
	let temp = from_locale_raw(lang, core.current_lang.as_str(), read_id);
	if temp.is_some() {
		temp
	} else {
		from_locale_raw(lang, core.default_lang.as_str(), read_id)
	}
}

impl Plugin for FluentInitial {
	fn build(&self, app: &mut App) {
        app
			.add_plugins(FluentPlugin)
			.insert_resource(code("zh-CN"))
			.add_systems(Startup, lang_init)
			.add_systems(Update, bundle_open_refresh)
			.add_systems(Update, blocking_to_opening);
	}
}
