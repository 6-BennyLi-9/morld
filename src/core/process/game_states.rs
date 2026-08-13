use bevy::prelude::{Message, Resource, States};

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, States)]
pub enum MorldStates {
	#[default]
	INITIALIZING,
	EXIT,
	ERR,
	MENU,
}

#[derive(Resource, Debug, Default)]
pub struct Errors{
	pub errors: Vec<String>
}

#[derive(Message, Debug, Default)]
pub struct OnError;
