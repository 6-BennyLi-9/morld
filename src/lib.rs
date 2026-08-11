mod util;

#[cfg(test)]
mod test{
	use crate::util::types::VecEx;
	use bevy::reflect::list::List;

	#[test]
	fn types_override() {
		let mut vec = VecEx::new();
		vec.0.push(1);
		vec.0.push(2);
		vec.0.push(3);

		println!("{}", vec)
	}
}
