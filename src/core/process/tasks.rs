use std::collections::HashSet;
use bevy::prelude::Resource;
use crate::core::process::tasks::InitialTasks::LoadLocales;

#[derive(Eq, Hash, PartialEq)]
pub enum InitialTasks{
	LoadLocales,
}

#[derive(Resource)]
pub struct MorldTasks {
	pub init: HashSet<InitialTasks>,
}

impl Default for MorldTasks {
	fn default() -> Self {
		let mut res = MorldTasks{
			init: HashSet::new(),
		};

		for init_item in [
			LoadLocales
		] {
			res.init.insert(init_item);
		}

		res
	}
}
