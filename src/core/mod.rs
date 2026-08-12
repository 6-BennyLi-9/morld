use crate::core::locales::FluentInitial;
use crate::core::types::ResInitial;
use bevy::app::{PluginGroup, PluginGroupBuilder};

pub mod types;
pub mod locales;
pub mod process;

pub struct MorldCore;

impl PluginGroup for MorldCore {
	fn build(self) -> PluginGroupBuilder {
		PluginGroupBuilder::start::<Self>()
			.add(ResInitial)
			.add(FluentInitial)
	}
}