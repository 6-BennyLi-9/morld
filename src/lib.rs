mod core;

#[cfg(test)]
#[allow(dead_code)]
pub mod test{
	use bevy::prelude::Res;
	use crate::core::types::MorldCoreRuntime;

	#[allow(dead_code)]
	pub fn print(
		core: Res<MorldCoreRuntime>
	){
		for item in &core.key_final{
			println!("{:?} ", item);
		}
	}
}
