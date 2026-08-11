use bevy::app::{PluginGroup, PluginGroupBuilder};
use crate::core::types::ResInitial;

pub mod types;

pub struct MorldCore;

impl PluginGroup for MorldCore {
	fn build(self) -> PluginGroupBuilder {
		PluginGroupBuilder::start::<Self>()
			.add(ResInitial)
	}
}