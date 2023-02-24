use std::cell::RefCell;
use std::fs::File;

pub struct Board {
	matrix: RefCell<[[i8; 9]; 9]>,
}

impl Board {}
