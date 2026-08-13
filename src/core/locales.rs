use crate::core::process::tasks::InitialTasks::LoadLocales;
use crate::core::process::tasks::MorldTasks;
use bevy::app::{App, Plugin, Startup};
use bevy::prelude::{Res, ResMut, Resource};
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::ops::Add;
use std::path::Path;
use sys_locale::get_locale;

pub struct LocalePlugin;

#[derive(Resource, Default)]
pub struct LocaleSettings {
	pub default_lang: String,
	pub current_lang: String,
}
/// 用于定位翻译条目
#[derive(Resource, Default)]
pub struct Localization{

	/// < locale code < item title, item value > >
	contents: HashMap<String, HashMap<String, String>>,
}

pub fn load_locales(
	mut localization: ResMut<Localization>,
	mut tasks: ResMut<MorldTasks>
) {
	localization.contents.insert(String::from("zh-CN"), HashMap::from([
		(String::from("debug"), String::from("DEBUG-zh-CN")),
	]));

	tasks.init.remove(&LoadLocales);
}

impl Localization {
	pub fn content(&self,id: String, settings: Res<LocaleSettings>) -> Result<String, String> {
		if self.contents.contains_key(settings.current_lang.as_str()) && self.contents[settings.current_lang.as_str()].contains_key(id.as_str()) {
			Ok(self.contents[settings.current_lang.as_str()][id.as_str()].clone()) // 成功读取 current
		} else if self.contents.contains_key(settings.default_lang.as_str()) && self.contents[settings.default_lang.as_str()].contains_key(id.as_str()){
			Ok(self.contents[settings.default_lang.as_str()][id.as_str()].clone()) // 成功读取 default
		} else {
			Err(format!("cannot spot value: {}", id))
		}
	}
}

impl Plugin for LocalePlugin {
	fn build(&self, app: &mut App) {
        app
			.insert_resource(LocaleSettings{
				default_lang: String::from("zh-CN"),
				current_lang: get_locale().unwrap_or_else(|| String::from("zh-CN")),
				..Default::default()
			})
			.insert_resource(Localization::default())
			.add_systems(Startup, load_locales);
	}
}
