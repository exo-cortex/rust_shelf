use std::fmt;

use crate::board;

#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Depth {
    Normal,
    Extra,
}

#[allow(dead_code)]
struct Compartment {
    height: f64,
    width: f64,
    depth: f64,
    backwall: Option<f64>,
}

#[allow(dead_code)]
impl Compartment {
    pub fn new(height: f64, width: f64, depth: f64, backwall: Option<f64>) -> Self {
        Compartment {
            height: height,
            width: width,
            depth: depth,
            backwall: backwall,
        }
    }
}

#[allow(dead_code)]
pub struct Shelf {
	shelf_width: f64,
    shelf_height: f64,
    shelf_normal: f64,
    shelf_extra: f64,
    board_thickness: f64,
    compartments: Vec<Vec<Compartment>>,
    level_heights: Vec<f64>,
    compartment_widths: Vec<Vec<f64>>,
    compartment_dephts: Vec<Vec<Depth>>,
    horizontal_main_boards: Vec<board::Board>,
    horizontal_extension_boards: Vec<board::Board>,
    vertical_boards: Vec<board::Board>,
    levels: usize,
    current_depth: Depth,
}

#[allow(dead_code)]
impl Shelf {
    pub fn new(
		width: f64,
        height: f64,
        depth: f64,
        shelf_extra: f64,
        board_thickness: f64,
    ) -> Self {
        Shelf {
			shelf_width: width,
            shelf_height: height,
            shelf_normal: depth,
            shelf_extra: shelf_extra,
            board_thickness: board_thickness,
            compartments: Vec::new(),
            level_heights: Vec::new(),
            compartment_widths: Vec::new(),
            compartment_dephts: Vec::new(),
            horizontal_main_boards: Vec::new(),
            horizontal_extension_boards: Vec::new(),
            vertical_boards: Vec::new(),
            levels: 0,
            current_depth: Depth::Normal,
        }
    }

    pub fn add_level(&mut self, level_height: f64) -> &mut Shelf {
        self.level_heights.push(level_height);
        self.compartment_widths.push(Vec::new());
        self.compartment_dephts.push(Vec::new());
        self.levels = self.level_heights.len();
        self
    }

    pub fn normal(&mut self) -> &mut Shelf {
        self.current_depth = Depth::Normal;
        self
    }

    pub fn deep(&mut self) -> &mut Shelf {
        self.current_depth = Depth::Extra;
        self
    }

    pub fn add_compartments(&mut self, widths: &[f64]) -> &mut Shelf {
        if self.compartment_widths.len() <= self.levels - 1 {
            self.compartment_widths.push(Vec::new());
            self.compartment_dephts.push(Vec::new());
        }
        self.compartment_widths[self.levels - 1].extend_from_slice(&widths);
        let depths = vec![self.current_depth; widths.len()];
        self.compartment_dephts[self.levels - 1].extend(depths);
        self
    }

    pub fn extend_to_full_width(&mut self) -> &mut Shelf {
        let total_width: f64 = self.compartment_widths[self.levels - 1].iter().sum::<f64>()
            + self.board_thickness * (self.compartment_widths[self.levels - 1].len() + 1) as f64;
        if total_width < self.shelf_width {
            let fill_width = self.shelf_width - total_width - self.board_thickness;
            if self.compartment_widths.len() == self.levels {
                self.compartment_widths[self.levels - 1].push(fill_width);
                self.compartment_dephts[self.levels - 1].push(self.current_depth);
            } else {
                self.compartment_widths[self.levels - 1].extend_from_slice(&[fill_width]);
                self.compartment_dephts[self.levels - 1].extend_from_slice(&[self.current_depth]);
            }
        }
        self
    }
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
                        if self.compartment_dephts[l][c] == Depth::Extra {
                            depth = "x".to_owned();
                        }
                        write!(f, "{}{}, ", &self.compartment_widths[l][c], depth).unwrap();
                    }
                    let mut depth: String = "".to_owned();
                    if self.compartment_dephts[l][self.compartment_dephts[l].len() - 1]
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
