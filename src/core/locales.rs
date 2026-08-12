use bevy::app::{App, Plugin, Startup};
use bevy::asset::LoadedFolder;
use bevy::prelude::{AssetServer, Commands, Handle, Res, Resource};
use bevy_fluent::{FluentPlugin, Locale};
use unic_langid::langid;

pub struct FluentInitial;

#[derive(Resource)]
pub struct LocaleFolder(Handle<LoadedFolder>);

pub fn load_locales(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
) {
	let handle = asset_server.load_folder("locales");
	commands.insert_resource(LocaleFolder(handle));
}

impl Plugin for FluentInitial {
	fn build(&self, app: &mut App) {
        app
			.add_plugins(FluentPlugin)
			.insert_resource(Locale::new(langid!("en-US")))
			.insert_resource(Locale::new(langid!("zh-CN")).with_default(langid!("en-US")))
			.add_systems(Startup, load_locales);
	}
}
