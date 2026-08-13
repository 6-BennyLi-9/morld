use crate::core::locales::LocalePlugin;
use crate::core::types::ResourcePlugin;
use bevy::app::{PluginGroup, PluginGroupBuilder};
use bevy::camera::Camera2d;
use bevy::prelude::Commands;
use crate::core::process::MorldProcessPlugin;
use crate::core::ui::UIPlugin;

pub mod types;
pub mod locales;
pub mod process;
pub mod ui;

pub struct MorldCore;

impl PluginGroup for MorldCore {
	fn build(self) -> PluginGroupBuilder {
		PluginGroupBuilder::start::<Self>()
			.add(ResourcePlugin)
			.add(MorldProcessPlugin)
			.add(LocalePlugin)
			.add(UIPlugin)
	}
}