use bevy::asset::LoadState;
use bevy::prelude::*;
use bevy_fluent::BundleAsset;
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

#[derive(Resource, Default)]
pub struct MorldLang{
	pub lang_handlers: HashMap<String, Handle<BundleAsset>>,
	pub lang_bundles: HashMap<String, BundleAsset>,
}

#[derive(Message, Default, Clone)]
pub struct BundleOpening{
	pub lang: String,
	pub handler:Handle<BundleAsset>,
}
#[derive(Message, Default, Clone)]
pub struct BundleBlocking{
	pub lang: String,
	pub handler:Handle<BundleAsset>,
}

impl BundleOpening{
	pub fn to_blocking(&self) -> BundleBlocking {
		BundleBlocking{
			lang: self.lang.clone(),
			handler: self.handler.clone(),
		}
	}
}
impl BundleBlocking{
	pub fn to_opening(&self) -> BundleOpening {
		BundleOpening{
			lang: self.lang.clone(),
			handler: self.handler.clone(),
		}
	}
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
		self.mana_current() as f64 / self.mana_maximum() as f64
	}

	fn restore_mana(&self, amount: u32);

	/// 如果目标不适配扩大法力值词条，返回 Err
	fn mana_expand(&self, amount: u32) -> Result<>;
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

fn lang_init(
	core: Res<MorldCoreRuntime>,
	mut lang: ResMut<MorldLang>,
	asset_server: ResMut<AssetServer>,
	mut sender: MessageWriter<BundleOpening>
){
	let handle = asset_server.load(format!("localization/{}.ftl.yml", core.default_lang.clone()));
	lang.lang_handlers.insert(core.default_lang.clone(), handle.clone());
	sender.write(BundleOpening{
		handler: handle,
		lang: core.default_lang.clone(),
	});
	let handle = asset_server.load(format!("localization/{}.ftl.yml", core.current_lang.clone()));
	lang.lang_handlers.insert(core.current_lang.clone(), handle.clone());
	sender.write(BundleOpening{
		handler: handle,
		lang: core.current_lang.clone(),
	});
}
fn bundle_open_refresh(
	mut targets: MessageReader<BundleOpening>,
	mut incomplete: MessageWriter<BundleBlocking>,
	asset_server: Res<AssetServer>,
	assets: Res<Assets<BundleAsset>>,
	mut lang: ResMut<MorldLang>,
){
	for bundle in targets.read() {
		if let Some(LoadState::Loaded) = asset_server.get_load_state(&bundle.handler) {
			lang.lang_bundles.insert(bundle.lang.clone(), assets.get(&bundle.handler).unwrap().clone());
		} else {
			incomplete.write(bundle.to_blocking());
		}
	}
}

fn blocking_to_opening(
	mut from: MessageReader<BundleBlocking>,
	mut to: MessageWriter<BundleOpening>,
){
	for bundle in from.read() {
		to.write(bundle.to_opening());
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
			.add_message::<BundleOpening>()
			.add_message::<BundleBlocking>()
			.insert_resource(MorldLang{
				..Default::default()
			})
			.add_systems(Startup, lang_init)
			.add_systems(Update, runtime_update)
			.add_systems(Update, key_input_update)
			.add_systems(Update, bundle_open_refresh)
			.add_systems(Update, blocking_to_opening);
	}
}
