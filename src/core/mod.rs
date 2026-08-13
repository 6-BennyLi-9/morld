use crate::core::locales::LocalePlugin;
use crate::core::types::ResourcePlugin;
use bevy::app::{PluginGroup, PluginGroupBuilder};
use crate::core::process::MorldProcessPlugin;

pub mod types;
pub mod locales;
pub mod process;

pub struct MorldCore;

impl PluginGroup for MorldCore {
	fn build(self) -> PluginGroupBuilder {
		PluginGroupBuilder::start::<Self>()
			.add(ResourcePlugin)
			.add(MorldProcessPlugin)
			.add(LocalePlugin)
	}
}