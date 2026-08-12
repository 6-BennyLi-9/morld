use bevy::app::{PluginGroup, PluginGroupBuilder};
use crate::core::locales::FluentInitial;
use crate::core::types::ResInitial;

pub mod types;
mod locales;

pub struct MorldCore;

impl PluginGroup for MorldCore {
	fn build(self) -> PluginGroupBuilder {
		PluginGroupBuilder::start::<Self>()
			.add(ResInitial)
			.add(FluentInitial)
	}
}