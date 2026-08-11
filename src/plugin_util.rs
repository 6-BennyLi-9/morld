use bevy::app::{App, Plugin};
use bevy::prelude::Component;
use std::marker::PhantomData;

///
pub struct MorldPlugin<T> {
	pub phantom_t: PhantomData<T>,
}

/// Implementation of `MorldPlugin` as a Plugin with
/// a generic type parameter of the `Component` trait.
impl<T: Component> Plugin for MorldPlugin<T> {
	fn build(&self, app: &mut App) {
		// app.add_systems(Startup, example_system::<T>);
	}
}

impl<T> MorldPlugin<T> {
	pub fn new() -> Self {
		Self::default()
	}
}
impl<T> Default for MorldPlugin<T> {
	fn default() -> Self {
		Self {
			phantom_t: PhantomData,
		}
	}
}
