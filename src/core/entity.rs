use crate::core::process::game_states::MorldStates;
use bevy::app::App;
use bevy::math::ops::sqrt;
use bevy::math::{Vec2, Vec3};
use bevy::prelude::{Commands, Component, OnEnter, Plugin, Query, ResMut, Resource, Transform, Update};

///玩家标识
#[derive(Component)]
pub struct Player;

#[derive(Resource, Debug, Default)]
pub struct EntityIdGen{
	id: u32,
}

impl EntityIdGen{
	pub fn next(&mut self) -> u32{
		self.id += 1;
		self.id
	}
}

#[derive(Component, Debug, Default)]
pub struct MFactor{
	pub val: f32,
	pub from: String,
}

impl MFactor{
	pub fn new(val: f32, from: String) -> MFactor{ MFactor{val, from } }
}

#[derive(Component, Debug, Default)]
pub struct MData{
	pub basic: f32,
	pub additions: Vec<MFactor>,
	pub add_multipliers: Vec<MFactor>,
	pub times_multipliers: Vec<MFactor>,
}

impl MData{
	pub fn new(basic: f32) -> Self{
		Self{
			basic,
			..Default::default()
		}
	}
}

impl MData{
	pub fn final_value(&self) -> f32{
		(self.basic + self.additions.iter().map(|x| x.val).sum::<f32>())
			* (self.add_multipliers.iter().map(|x| x.val).sum::<f32>() + 1f32)
			* (self.times_multipliers.iter().map(|x| x.val + 1f32).product::<f32>())
	}
}

///定义标准实体
#[derive(Component, Debug, Default)]
pub struct Entity{
	pub id: u32,
	pub pos: Vec2,
}

impl Entity{
	pub fn new(id: u32, pos: Vec2) -> Self{
		Self{
			id,
			pos,
		}
	}
}

///标识携带生命值的实体
#[derive(Component, Debug, Default)]
pub struct Carnal{
	pub health: f32,
	pub health_maximum: f32,

	pub amour: MData,
	pub magic_resistance: MData,
}

impl Carnal{
	pub fn new(health: f32, health_maximum: f32, amour: f32, magic_resistance: f32) -> Self{
		Self{
			health,
			health_maximum,
			amour: MData::new(amour),
			magic_resistance: MData::new(magic_resistance),
		}
	}
}

///标识携带法力值的实体
#[derive(Component, Debug, Default)]
pub struct Mage{
	pub mana: f32,
	pub mana_maximum: f32,
}
impl Mage{
	pub fn new(mana: f32, mana_maximum: f32) -> Self{
		Self{
			mana,
			mana_maximum,
		}
	}
}

fn update_transform(
	mut query: Query<(&mut Transform, &Entity)>,
){
	for mut item in &mut query{
		item.0.translation = Vec3::new(
			item.1.pos.x,
			item.1.pos.y / sqrt(2.0),
			0.0,
		);
	}
}

pub struct EntityPlugin;
impl Plugin for EntityPlugin {
	fn build(&self, app: &mut App) {
		app
			.add_systems(OnEnter(MorldStates::GAMING),|mut commands: Commands, mut entity_id_gen: ResMut<EntityIdGen>|{
				commands.spawn((
					Entity::new(entity_id_gen.next(), Vec2::ZERO),
					Carnal::new(100.0, 100.0, 10.0, 10.0),
					Mage::new(0.0, 0.0),
					Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
					Player
					));
			})
			.add_systems(Update, update_transform);
	}
}
