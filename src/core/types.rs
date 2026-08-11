use bevy::prelude::*;
use std::collections::HashMap;

///核心运行时
#[derive(Resource, Default, Debug)]
struct MorldCoreRuntime {
	fps: u32,
	///按下的按键以及按下的时间戳
	key_input: HashMap<KeyCode, f64>
}

///定义标准实体
#[derive(Component, Debug)]
pub struct Entity{
	id: u32,
	pos: Vec2,
}

///标识携带生命值的实体
trait WithHealth{
	fn health_current(&self) -> u32;
	fn health_maximum(&self) -> u32;
	#[inline]
	fn health_percentage(&self) -> f64 {
		self.health_current() as f64 / self.health_maximum() as f64
	}

	/// returns: u32
	/// - 溢出的治疗量
	fn heal(&self, amount: u32) -> u32;

	/// 如果目标不适配扩大生命值词条，返回 Err
	fn health_expand(&self, amount: u32) -> Result<>;
}

///标识携带法力值的实体
trait WithMana{
	fn mana_current(&self) -> u32;
	fn mana_maximum(&self) -> u32;
	#[inline]
	fn mana_percentage(&self) -> f64 {
		self.health_current() as f64 / self.health_maximum() as f64
	}

	fn restore_mana(&self, amount: u32);

	/// 如果目标不适配扩大法力值词条，返回 Err
	fn mana_expand(&self, amount: u32) -> Result<>;
}

fn runtime_update(
	core: Res<MorldCoreRuntime>,
	time: Res<Time>,
){
	core.fps = (1f64 / time.delta_secs_f64()) as u32;
}
fn key_input_update(
	core: ResMut<MorldCoreRuntime>,
	input: Res<ButtonInput<KeyCode>>,
	time: Res<Time>,
){
	for item in input.get_just_pressed(){
		core.key_input.insert(*item, time.elapsed_secs_f64());
	}
	for item in input.get_just_released(){
		core.key_input.remove(item);
	}
}

pub struct ResInitial;
impl Plugin for ResInitial {
	fn build(&self, app: &mut App) {
		app
			.init_resource::<MorldCoreRuntime>()
			.add_systems(Update, runtime_update)
			.add_systems(Update, key_input_update);
	}
}
