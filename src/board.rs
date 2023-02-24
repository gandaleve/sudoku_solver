use std::cell::RefCell;
use std::fs::File;

pub struct Board {
	matrix: RefCell<[[u8; 9]; 9]>,
}

impl Board {
	fn valid_guess(&self, row: usize, col: usize, val: u8) -> bool {
		let m = self.matrix.borrow();

		for cell in 0..9 {
			if m[row][cell] == val || m[cell][col] == val {
				return false;
			}
		}

		let u = (row / 3) * 3;
		let v = (col / 3) * 3;

		for x in &m[u..u + 3] {
			for y in v..v + 3 {
				if x[y] == val {
					return false;
				}
			}
		}
		true
	}

	fn next_empty(&self) -> Option<(usize, usize)> {
		let m = self.matrix.borrow();
		for row in 0..9 {
			for cell in 0..9 {
				if m[row][cell] == 0 {
					return Some((row, cell));
				}
			}
		}
		None
	}

	fn guess(&self, row: usize, col: usize) -> Vec<u8> {
		let mut guesses: Vec<u8> = vec![];
		for guess in 1..=9 {
			if self.valid_guess(row, col, guess) {
				guesses.push(guess)
			}
		}
		guesses
	}
	fn solve_helper(&self) -> bool {
		if let Some((x, y)) = self.next_empty() {
			for guess in self.guess(x, y) {
				self.matrix.borrow_mut()[x][y] = guess;
				if !self.solve_helper() {
					self.matrix.borrow_mut()[x][y] = 0
				}
			}
			if self.matrix.borrow_mut()[x][y] == 0 {
				return false;
			}
		}
		true
	}

	pub fn print(&self) {
		let m = self.matrix.borrow();
		println!("╭───────┬───────┬───────╮");
		for u in 0..9 {
			for v in 0..9 {
				if v == 0 {
					print!("│")
				}
				if m[u][v] == 0 {
					print!("  ");
				} else {
					print!(" {}", m[u][v]);
				}
				if (v + 1) % 3 == 0 {
					print! {" │"}
				}
			}
			if (u + 1) % 3 == 0 && u != 8 {
				println!("\n├───────┼───────┼───────┤");
			} else {
				println!();
			}
		}
		println!("╰───────┴───────┴───────╯");
	}
}

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
