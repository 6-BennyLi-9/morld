use std::fmt;
use std::fmt::Display;

pub struct VecEx<T>(pub Vec<T>);
impl<T: Display> Display for VecEx<T> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "[")?;
		for (i, item) in self.0.iter().enumerate() {
			if i > 0 {
				write!(f, ", ")?;
			}
			write!(f, "{}", item)?;
		}
		write!(f, "]")
	}
}
impl<T: Default> Default for VecEx<T> {
	fn default() -> Self {
		VecEx(Default::default())
	}
}
impl<T: Default> VecEx<T> {
	pub fn new() -> Self {
		Self::default()
	}
}