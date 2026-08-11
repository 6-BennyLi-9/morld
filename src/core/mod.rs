use bevy::app::{PluginGroup, PluginGroupBuilder};

pub mod types;

pub struct MorldCore;

impl PluginGroup for MorldCore {
	fn build(self) -> PluginGroupBuilder {
		PluginGroupBuilder::start::<Self>()
	}
}