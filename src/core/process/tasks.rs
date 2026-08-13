use std::collections::HashSet;
use bevy::prelude::Resource;

#[derive(Eq, Hash, PartialEq)]
pub enum InitialTasks{
	LoadLocales,
	LoadAudio,
	CameraSetup,
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
			InitialTasks::LoadLocales,
			InitialTasks::LoadAudio,
			InitialTasks::CameraSetup,
		] {
			res.init.insert(init_item);
		}

		res
	}
}
