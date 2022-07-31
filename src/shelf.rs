struct Compartment {
    height: f64,
    width: f64,
    depth: f64,
    backwall: Option<f64>,
}

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
    shelf_height: f64,
    shelf_width: f64,
    shelf_normal_depth: f64,
    shelf_extra_depth: f64,
    board_thickness: f64,
    compartments: Vec<Vec<Compartment>>,
    level_heights: Vec<f64>,
    compartment_widths: Vec<Vec<f64>>,
}

impl Shelf {
    pub fn new(
        height: f64,
        width: f64,
        board_thickness: f64,
        depth: f64,
        shelf_extra_depth: f64,
    ) -> Self {
        Shelf {
            shelf_height: height,
            shelf_width: width,
            shelf_normal_depth: depth,
            board_thickness: board_thickness,
            shelf_extra_depth: shelf_extra_depth,
            compartments: Vec::new(),
            level_heights: Vec::new(),
            compartment_widths: Vec::new(),
        }
    }

    // pub fn add_level(&mut self, height: f64) {}
    // pub fn add_level_with_compartments(&mut self, widths: &[f64]) {}
    // pub fn add_compartments_at_level(&mut self, at_level: usize, widths: &[f64]) {}
}
