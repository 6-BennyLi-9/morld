use crate::core::audio::AudioPlugin;
use crate::core::locales::LocalePlugin;
use crate::core::process::MorldProcessPlugin;
use crate::core::core_res::ResourcePlugin;
use crate::core::ui::UIPlugin;
use bevy::app::{PluginGroup, PluginGroupBuilder};
use crate::core::entity::PlayerPlugin;

pub mod core_res;
pub mod locales;
pub mod process;
pub mod ui;
pub mod audio;
pub mod entity;

pub struct MorldCore;

impl PluginGroup for MorldCore {
	fn build(self) -> PluginGroupBuilder {
		PluginGroupBuilder::start::<Self>()
			.add(ResourcePlugin)
			.add(MorldProcessPlugin)
			.add(LocalePlugin)
			.add(UIPlugin)
			.add(AudioPlugin)
			.add(PlayerPlugin)
	}
}