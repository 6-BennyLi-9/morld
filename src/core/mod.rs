use crate::core::audio::AudioPlugin;
use crate::core::locales::LocalePlugin;
use crate::core::process::MorldProcessPlugin;
use crate::core::types::ResourcePlugin;
use crate::core::ui::UIPlugin;
use bevy::app::{PluginGroup, PluginGroupBuilder};

pub mod types;
pub mod locales;
pub mod process;
pub mod ui;
pub mod audio;

pub struct MorldCore;

impl PluginGroup for MorldCore {
	fn build(self) -> PluginGroupBuilder {
		PluginGroupBuilder::start::<Self>()
			.add(ResourcePlugin)
			.add(MorldProcessPlugin)
			.add(LocalePlugin)
			.add(UIPlugin)
			.add(AudioPlugin)
	}
}