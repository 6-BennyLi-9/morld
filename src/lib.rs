mod core;

mod test{
	use bevy::input::ButtonInput;
	use bevy::prelude::{KeyCode, Res, ResMut, Time};
	use crate::core::types::MorldCoreRuntime;

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
}
