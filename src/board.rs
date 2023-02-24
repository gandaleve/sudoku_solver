use std::cell::RefCell;
use std::fs::File;

pub struct Board {
	matrix: RefCell<[[u8; 9]; 9]>,
}

impl Board {}

pub struct BoardBuilder {
	matrix: RefCell<[[u8; 9]; 9]>,
}
impl BoardBuilder {
	pub fn file(&self, fd: File) {
		todo!()
	}
	pub fn array(&self, val: [[u8; 9]; 9]) {
		self.matrix.swap(&RefCell::new(val));
	}
	pub fn build(self) -> Board {
		Board {
			matrix: self.matrix,
		}
	}
}
