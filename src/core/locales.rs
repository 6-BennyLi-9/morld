use bevy::app::{App, Plugin, Startup};
use bevy::asset::{LoadState, LoadedFolder};
use bevy::prelude::{AssetServer, Commands, Handle, Res, Resource, Update};
use bevy_fluent::{FluentPlugin, Locale, LocalizationBuilder};
use unic_langid::langid;

pub struct FluentInitial;

/// The folder of locales. Const.
#[derive(Resource)]
pub struct LocaleFolder(Handle<LoadedFolder>);

pub fn load_locales(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
) {
	let handle = asset_server.load_folder("locales");
	commands.insert_resource(LocaleFolder(handle));
}

pub fn update_localizations(
	mut commands: Commands,
	localization_builder: LocalizationBuilder,
	asset_server: Res<AssetServer>,
	locale_folder: Res<LocaleFolder>,
) {
	if let Some(LoadState::Loaded) = asset_server.get_load_state(&locale_folder.0) {
		let localization = localization_builder.build(&locale_folder.0);
		commands.remove_resource::<LocaleFolder>();
		commands.insert_resource(localization);
	}
}

impl Plugin for FluentInitial {
	fn build(&self, app: &mut App) {
        app
			.add_plugins(FluentPlugin)
			.insert_resource(Locale::new(langid!("en-US")))
			.insert_resource(Locale::new(langid!("zh-CN")).with_default(langid!("en-US")))
			.add_systems(Startup, load_locales)
			.add_systems(Update, update_localizations);
	}
}
