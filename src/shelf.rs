use rand::{prelude::*, seq::SliceRandom};
use std::fmt;

use crate::board;
// use crate::compartment;

#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Depth {
	Normal,
	Extra,
}

#[allow(dead_code)]
pub struct Shelf {
	shelf_width: f64,
	shelf_height: f64,
	shelf_normal: f64,
	shelf_extra: f64,
	board_thickness: f64,
	level_heights: Vec<f64>,
	compartment_widths: Vec<Vec<f64>>,
	compartment_depths: Vec<Vec<Depth>>,
	horizontal_main_boards: Vec<board::Board>,
	horizontal_extension_boards: Vec<board::Board>,
	vertical_boards: Vec<board::Board>,
	rng: ThreadRng,
}

#[allow(dead_code)]
impl Shelf {
	pub fn new(
		shelf_width: f64,
		shelf_height: f64,
		shelf_depth: f64,
		shelf_extra: f64,
		board_thickness: f64,
	) -> Self {
		Shelf {
			shelf_width,
			shelf_height,
			shelf_normal: shelf_depth,
			shelf_extra,
			board_thickness: board_thickness,
			level_heights: Vec::new(),
			compartment_widths: Vec::new(),
			compartment_depths: Vec::new(),
			horizontal_main_boards: Vec::new(),
			horizontal_extension_boards: Vec::new(),
			vertical_boards: Vec::new(),
			rng: thread_rng(),
		}
	}

	pub fn add_level(&mut self, level_height: f64) -> &mut Shelf {
		self.level_heights.push(level_height);
		self.compartment_widths.push(Vec::new());
		self.compartment_depths.push(Vec::new());
		self
	}

	pub fn add_level_to_full_height(&mut self) -> &mut Shelf {
		let total_height: f64 = self.level_heights.iter().sum::<f64>()
			+ self.board_thickness * (self.level_heights.len() + 1) as f64;
		self.level_heights.push(total_height);
		self.compartment_widths.push(Vec::new());
		self.compartment_depths.push(Vec::new());
		self
	}

	pub fn add_compartments(&mut self, widths: &[f64], depth: Depth) -> &mut Shelf {
		if self.compartment_widths.is_empty()
			|| self.compartment_widths.len() < self.level_heights.len()
		{
			panic!("create new level with `add_level(f64)` before adding compartment widths.");
		}
		self.compartment_widths
			.last_mut()
			.unwrap()
			.extend_from_slice(&widths);
		let depths = vec![depth; widths.len()];
		self.compartment_depths.last_mut().unwrap().extend(depths);
		self
	}

	pub fn extend_to_full_width(&mut self, depth: Depth) -> &mut Shelf {
		let total_width: f64 = self.compartment_widths.last().unwrap().iter().sum::<f64>()
			+ self.board_thickness * (self.compartment_widths.last().unwrap().len() + 1) as f64;
		if total_width < self.shelf_width {
			let fill_width = self.shelf_width - total_width - self.board_thickness;
			if self.compartment_widths.len() == self.level_heights.len() {
				self.compartment_widths.last_mut().unwrap().push(fill_width);
				self.compartment_depths.last_mut().unwrap().push(depth);
			} else {
				self.compartment_widths
					.last_mut()
					.unwrap()
					.extend_from_slice(&[fill_width]);
				self.compartment_depths
					.last_mut()
					.unwrap()
					.extend_from_slice(&[depth]);
			}
		}
		self
	}


	fn horizontal_extension_boards(&mut self) {
		let mut first_board = board::Board::new(self.shelf_width, self.shelf_normal, self.board_thickness);
		let mut intervals: Vec<Vec<(f64, f64)>> = Vec::new();
		for (lw, ld) in self.compartment_widths.iter().zip(&self.compartment_depths) {
			let mut x: f64 = 0.0;
			let mut temp: Vec<(f64, f64)> = Vec::new();
			for (cw, cd) in lw.iter().zip(ld) {
				if cd == &Depth::Extra {
					temp.push((x, x + cw + self.board_thickness));
				}
				x += cw + self.board_thickness;
			}

		}
		
		// self.horizontal_main_boards.push();
	}
	fn vertical_boards(&self) {}
	fn extension_boards(&self) {}

	pub fn randomize_depths(&mut self) -> &mut Shelf {
		for level in &mut self.compartment_depths {
			level.shuffle(&mut self.rng);
		}
		self
	}

	pub fn compartments(&self) {} // return vector of compartments
}

impl fmt::Display for Shelf {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "+++ shelf configuration (in mm) +++\n").unwrap();
		write!(
			f,
			"width: {}, height: {}, main depth: {}, extra depth: {}\n",
			self.shelf_width, self.shelf_height, self.shelf_normal, self.shelf_extra
		)
		.unwrap();
		write!(f, "wood thickness: {}\n", self.board_thickness).unwrap();
		for l in 0..self.level_heights.len() {
			write!(
				f,
				"level: {}, height: {}, compartments: [",
				l, &self.level_heights[l]
			)
			.unwrap();
			// println!("{:?}, length = {}", self.compartment_widths[l], self.compartment_widths[l].len());
			if self.compartment_widths.len() > 0 {
				if self.compartment_widths[l].len() > 0 {
					let mut depth: String = "".to_owned();
					for c in 0..self.compartment_widths[l].len() - 1 {
						if self.compartment_depths[l][c] == Depth::Extra {
							depth = "x".to_owned();
						}
						write!(f, "{}{}, ", &self.compartment_widths[l][c], depth).unwrap();
					}
					let mut depth: String = "".to_owned();
					if self.compartment_depths[l][self.compartment_depths[l].len() - 1]
						== Depth::Extra
					{
						depth = "x".to_owned();
					}
					write!(
						f,
						"{}{}]",
						&self.compartment_widths[l][self.compartment_widths[l].len() - 1],
						depth
					)
					.unwrap();
				} else {
					write!(f, "None]").unwrap();
				}
			}
			let accumulated_width: f64 = self.compartment_widths[l].iter().sum::<f64>()
				+ self.board_thickness * (self.compartment_widths[l].len() + 1) as f64;
			write!(f, ", total combined width: {}\n", accumulated_width).unwrap();
		}
		write!(f, "+++++++++++++++++++++++++++++++++++").unwrap();
		Ok(())
	}
}
