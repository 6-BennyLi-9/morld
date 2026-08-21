pub mod status_buf;

use crate::core::entity::{Entity, EntityMovement, Player};
use crate::entity::status_buf::EntityStatus;
use crate::entity::status_buf::StatusBuff::Imprison;
use bevy::app::{App, Plugin};
use bevy::input::ButtonInput;
use bevy::prelude::{KeyCode, Query, Res, Time, Timer, Update, Vec2};
use bevy::prelude::KeyCode::{KeyA, KeyD, KeyS, KeyW};

fn player_movement(
	mut query: Query<(&mut Entity, &EntityStatus, &EntityMovement, &Player)>,
	time: Res<Time>,
	input: Res<ButtonInput<KeyCode>>,
){
	for (mut entity, status, movements, _) in query.iter_mut() {
		if !status.list.contains_key(&Imprison){
			if input.pressed(KeyW){
				entity.pos.y += time.delta_secs() * movements.speed.final_value();
			}
			if input.pressed(KeyS){
				entity.pos.y -= time.delta_secs() * movements.speed.final_value();
			}
			if input.pressed(KeyA){
				entity.pos.x -= time.delta_secs() * movements.speed.final_value();
			}
			if input.pressed(KeyD){
				entity.pos.x += time.delta_secs() * movements.speed.final_value();
			}
		}
	}
}

pub struct EntityCore;
impl Plugin for EntityCore{
	fn build(&self, app: &mut App) {
		app.
			add_systems(Update, player_movement);
	}
}