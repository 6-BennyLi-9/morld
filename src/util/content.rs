use std::fmt::{Display, Formatter};
use crate::util::types::VecEx;

/// The contents of the game
#[derive(Debug)]
pub struct ContentPixel<T>{
	index: u64,
	title: String,
	val: Box<T>,
}

impl<T> Display for ContentPixel<T>
where T: Display{
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "[{}][\"{}\" = {}]", self.index, self.title, self.val)
	}
}

impl<T> Default for ContentPixel<T>
where T: Default{
	fn default() -> Self {
		ContentPixel{
			index: 0,
			title: String::from("BLANK"),
			val: Box::new(T::default()),
		}
	}
}
impl<T> ContentPixel<T> {
	fn new(index: u64, val: Box<T>, title: String) -> Self {
		ContentPixel{
			index,
			title,
			val
		}
	}
}

///Factory of `ContentPixel`
pub trait Loadable<T>{
	fn load(&self, index: u64) -> ContentPixel<T>;
}
impl<T> Loadable<T> for T
where T: Display + Clone {
	fn load(&self, index: u64) -> ContentPixel<T> {
		ContentPixel::new(
			index,
			Box::new(self.clone()),
			String::from("Value"),
		)
	}
}


/// List of contents of the game
pub struct ContentList<T>
where T:Loadable<T> {
	buf: VecEx<ContentPixel<T>>,
	cnt: u64,
}

impl<T> ContentList<T>
where T:Loadable<T> + Default {
	pub fn load_content(&mut self, content: T) {
		self.buf.0.push(content.load(self.cnt.clone()));
		self.cnt += 1;
	}

	fn new() -> Self {
		Self::default()
	}
}

impl<T> Default for ContentList<T>
where T:Loadable<T> + Default{
	fn default() -> Self {
		ContentList{
			buf: VecEx::new(),
			cnt: 0,
		}
	}
}

impl<T> Display for ContentList<T>
where T:Loadable<T> + Display {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		self.buf.fmt(f)
	}
}