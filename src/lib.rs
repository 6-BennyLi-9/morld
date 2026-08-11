mod plugin_util;
mod util;

#[cfg(test)]
mod test{

	#[test]
	fn types_override() {
		let mut vec = super::util::content::ContentList::default();
		vec.load_content(1);
		vec.load_content(2);
		vec.load_content(3);

		println!("{}", vec)
	}
}
