use crate::core::process::game_states::{Errors, MorldStates};
use bevy::app::App;
use bevy::prelude::{Camera2d, Commands, Justify, LineBreak, OnEnter, Plugin, ResMut, Text2d, TextFont, TextLayout, Transform, default};
use bevy::text::FontSize;

pub struct MorldMenu;

fn on_error_behaviour(
	mut commands: Commands,
	errors: ResMut<Errors>,
){
	// commands.spawn(Camera2d);
	let mut text_y = 0f32;
	commands.spawn((
		Text2d::new("Interrupted because of errors below."),
		TextFont {
			font_size: FontSize::Px(30.0),
			..default()
		},
		TextLayout::new(Justify::Left, LineBreak::WordBoundary),
		Transform::from_xyz(0.0, 70.0, 1.0),
	));

	for e in &errors.errors{
		commands.spawn((
			Text2d::new(e),
			TextFont {
				font_size: FontSize::Px(30.0),
				..default()
			},
			TextLayout::new(Justify::Left, LineBreak::WordBoundary),
			Transform::from_xyz(0.0, text_y, 1.0),
		));

		text_y -= 35f32;
	}
}

impl Plugin for MorldMenu {
	fn build(&self, app: &mut App) {
		app
			.add_systems(OnEnter(MorldStates::ERR), on_error_behaviour);
	}
}