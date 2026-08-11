mod core;

#[cfg(test)]
pub mod test{
	use bevy::prelude::Res;
	use crate::core::types::MorldCoreRuntime;

	pub fn print(
		core: Res<MorldCoreRuntime>
	){
		for item in &core.key_final{
			println!("{:?} ", item);
		}
	}
}
