
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
pub enum HoleUsage {
    Normal, // normal hole
    Sink, // countersink for screwhead
}

#[derive(Debug)]
pub struct Hole {
    pub position: DrillSide,
    pub diameter: f64,
    pub drill_depth: f64,
    pub usage: Option<HoleUsage>,
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
            DrillSide::Front(mut x, mut y) => {
                wrap(&mut x, &self.width);
                wrap(&mut y, &self.height);
                actual_position = DrillSide::Front(x, y);
            }
            DrillSide::Back(mut x, mut y) => {
                wrap(&mut x, &self.width);
                wrap(&mut y, &self.height);
                actual_position = DrillSide::Back(x, y);
            }
            DrillSide::Left(mut y) => {
                wrap(&mut y, &self.height);
                actual_position = DrillSide::Left(y);
            }
            DrillSide::Right(mut y) => {
                wrap(&mut y, &self.height);
                actual_position = DrillSide::Right(y);
            }
            DrillSide::Top(mut x) => {
                wrap(&mut x, &self.width);
                actual_position = DrillSide::Top(x);
            }
            DrillSide::Bottom(mut x) => {
                wrap(&mut x, &self.width);
                actual_position = DrillSide::Bottom(x);
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

fn wrap(position: &mut f64, size: &f64) {
    if *position < 0.0 {
        *position += size
    };
}
