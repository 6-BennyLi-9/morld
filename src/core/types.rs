use bevy::prelude::*;
use std::collections::HashMap;
use sys_locale::get_locale;

///核心运行时
#[derive(Resource, Default, Debug)]
pub struct MorldCoreRuntime {
	pub fps: u32,
	///按下的按键以及按下的时间戳
	pub key_input: HashMap<KeyCode, f64>,
	///刚松开的键及其曾经按下的时间
	pub key_final: HashMap<KeyCode, f64>,

	pub default_lang: String,
	pub current_lang: String,
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
	pub fn final_value(&self) -> f32{
		(self.basic + self.additions.iter().map(|x| x.val).sum::<f32>())
			* (self.add_multipliers.iter().map(|x| x.val).sum::<f32>() + 1f32)
			* (self.times_multipliers.iter().map(|x| x.val + 1f32).product::<f32>())
	}
}

///定义标准实体
#[derive(Component, Debug, Default)]
pub struct Entity{
	id: u32,
	pos: Vec2,
}

///标识携带生命值的实体
#[derive(Component, Debug, Default)]
pub struct Carnal{
	health: f32,
	health_maximum: f32,

	amour: MData,
	magic_resistance: MData,
}

///标识携带法力值的实体
#[derive(Component, Debug, Default)]
pub struct Mage{
	mana: f32,
	mana_maximum: f32,
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

pub struct ResInitial;
impl Plugin for ResInitial {
	fn build(&self, app: &mut App) {
		app
			.insert_resource(MorldCoreRuntime{
				default_lang: String::from("zh-CN"),
				current_lang: get_locale().unwrap_or_else(|| String::from("zh-CN")),
				..Default::default()
			})
			.add_systems(Update, runtime_update)
			.add_systems(Update, key_input_update);
	}
}
