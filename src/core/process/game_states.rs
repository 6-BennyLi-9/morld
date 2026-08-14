use bevy::prelude::{Resource, States};

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, States)]
pub enum MorldStates {
	#[default]
	INITIALIZING,
	GAMING,
	ERR,
	MENU,
}

#[derive(Resource, Debug, Default)]
pub struct Errors{
	pub errors: Vec<String>
}
