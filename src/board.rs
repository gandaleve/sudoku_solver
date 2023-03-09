use std::cell::RefCell;
use std::fs::File;

pub struct Board {
	flat_vec: RefCell<Vec<u8>>,
}

impl Board {
	fn valid_guess(&self, row: usize, col: usize, val: u8) -> bool {
		let m = self.flat_vec.borrow();

		for cell in 0..9 {
			if m[row][cell] == val || m[cell][col] == val {
				return false;
			}
		}

		let u = (row / 3) * 3;
		let v = (col / 3) * 3;

		for x in &m[u..u + 3] {
			if x[v..v + 3].contains(&val) {
				return false;
			}
		}
		true
	}

	fn next_empty(&self) -> Option<(usize, usize)> {
		let m = self.flat_vec.borrow();
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
				self.flat_vec.borrow_mut()[x][y] = guess;
				if !self.solve_helper() {
					self.flat_vec.borrow_mut()[x][y] = 0
				}
			}
			if self.flat_vec.borrow_mut()[x][y] == 0 {
				return false;
			}
		}
		true
	}

	pub fn solve(&self) {
		println!("Solving....");
		self.print();
		self.solve_helper();
	}

	pub fn print(&self) {
		let m = self.flat_vec.borrow();
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
	flat_vec: Vec<u8>,
}
impl BoardBuilder {
	pub fn new() -> BoardBuilder {
		BoardBuilder {
			flat_vec: vec![0; 81],
		}
	}
	pub fn file(self, fp: &str) {
		let fd = fs::read_to_string(fp).unwrap();
		let mut line: Vec<u8> = vec![];
		for char in fd.chars() {
			match char {
				' ' | '\n' => continue,
				'_' => line.push(0),
				_ => line.push(char.to_digit(10).unwrap() as u8),
			}
		}
	}
	pub fn array(self, val: [[u8; 9]; 9]) -> BoardBuilder {
		self.matrix.swap(&RefCell::new(val));
		self
	}
	pub fn build(self) -> Board {
		Board {
			matrix: self.matrix,
		}
	}
}
