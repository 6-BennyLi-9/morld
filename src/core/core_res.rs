use bevy::prelude::*;
use std::collections::HashMap;
use crate::core::entity::EntityIdGen;

///核心运行时
#[derive(Resource, Default, Debug)]
pub struct MorldCoreRuntime {
	pub fps: u32,
	///按下的按键以及按下的时间戳
	pub key_input: HashMap<KeyCode, f64>,
	///刚松开的键及其曾经按下的时间
	pub key_final: HashMap<KeyCode, f64>,
}

impl MorldCoreRuntime{
	pub fn key_input_time(&self, key: KeyCode, time: f64) -> Result<f64, ()> {
		if self.key_input.contains_key(&key) {
			Ok(time - self.key_input[&key])
		} else if self.key_final.contains_key(&key) {
			Ok(self.key_final[&key])
		} else {
			Err(())
		}
	}
}

fn runtime_update(
	mut core: ResMut<MorldCoreRuntime>,
	time: Res<Time>,
){
	core.fps = (1f64 / time.delta_secs_f64()) as u32;
}
fn key_input_update(
	mut core: ResMut<MorldCoreRuntime>,
	input: Res<ButtonInput<KeyCode>>,
	time: Res<Time>,
){
	core.key_final.clear();
	for item in input.get_just_pressed(){
		core.key_input.insert(*item, time.elapsed_secs_f64());
	}
	for item in input.get_just_released(){
		let val = time.elapsed_secs_f64() - core.key_input.remove(item).unwrap();
		core.key_final.insert(*item, val);
	}
}

pub struct ResourcePlugin;
impl Plugin for ResourcePlugin {
	fn build(&self, app: &mut App) {
		app
			.insert_resource(MorldCoreRuntime::default())
			.insert_resource(EntityIdGen::default())
			.add_systems(Update, runtime_update)
			.add_systems(Update, key_input_update);
	}
}
