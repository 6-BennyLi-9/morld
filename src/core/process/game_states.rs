use bevy::prelude::States;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, States)]
pub enum MorldStates {
	#[default]
	INITIALIZING,
	EXIT,
	ERR,
	MENU,
}
