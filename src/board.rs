#[allow(dead_code)]
#[derive(Debug, Copy, Clone)]
pub enum DrillSide {
    Front(f64, f64),
    Back(f64, f64),
    Left(f64),
    Right(f64),
    Top(f64),
    Bottom(f64),
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum HolePurpose {
    Normal, // normal hole
    Sink,   // countersink for screwhead
}

#[derive(Debug)]
pub struct Hole {
    pub position: DrillSide,
    pub diameter: f64,
    pub drill_depth: f64,
    pub usage: Option<HolePurpose>,
}

#[allow(dead_code)]
pub struct Board {
    pub width: f64,
    pub height: f64,
    pub thickness: f64,
    pub holes: Vec<Hole>,
}

#[allow(dead_code)]
impl Board {
    pub fn new(width: f64, height: f64, thickness: f64) -> Self {
        Board {
            width: width,
            height: height,
            thickness: thickness,
            holes: Vec::<Hole>::new(),
        }
    }

    pub fn drill(&mut self, position: DrillSide, diameter: f64, depth: f64) {
        let actual_position: DrillSide;
        match position {
            DrillSide::Front(x, y) => {
                let actual_x = if x < 0.0 { x + &self.width } else { x };
                let actual_y = if y < 0.0 { y + &self.height } else { y };
                actual_position = DrillSide::Front(actual_x, actual_y);
            }
            DrillSide::Back(x, y) => {
                let actual_x = if x < 0.0 { x + &self.width } else { x };
                let actual_y = if y < 0.0 { y + &self.height } else { y };
                actual_position = DrillSide::Back(actual_x, actual_y);
            }
            DrillSide::Left(y) => {
                actual_position = if y < 0.0 {
                    DrillSide::Left(y + &self.height)
                } else {
                    DrillSide::Left(y)
                };
            }
            DrillSide::Right(y) => {
                actual_position = if y < 0.0 {
                    DrillSide::Right(y + &self.height)
                } else {
                    DrillSide::Right(y)
                };
            }
            DrillSide::Top(x) => {
                actual_position = if x < 0.0 {
                    DrillSide::Top(x + &self.width)
                } else {
                    DrillSide::Top(x)
                };
            }
            DrillSide::Bottom(x) => {
                actual_position = if x < 0.0 {
                    DrillSide::Bottom(x + &self.width)
                } else {
                    DrillSide::Bottom(x)
                };
            }
        };
        self.holes.push(Hole {
            position: actual_position,
            diameter: diameter,
            drill_depth: depth,
            usage: None,
        })
    }
}
