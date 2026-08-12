use crate::core::types::{MorldCoreRuntime, MorldLang};
use bevy::app::{App, Plugin, Update};
use bevy::prelude::{AssetServer, Handle, Local, Res};
use bevy_fluent::{BundleAsset, FluentPlugin, Locale};

pub struct FluentInitial;

fn code(locale_code: &str) -> Locale {
	Locale::new(locale_code.parse().expect("Failed to parse locale code"))
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
			.insert_resource(code("zh-CN"));
	}
}
