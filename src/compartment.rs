// work in progress
#[allow(dead_code)]
pub enum Content {
    SubShelf(f64),
    Drawer(f64),
    SlideShelf(usize, f64), // how many slide shelfs, depth
    DoorLeft,
    DoorRight,
}

#[allow(dead_code)]
pub struct Compartment {
    height: f64,
    width: f64,
    depth: f64,
    backwall: Option<f64>,
    content: Option<Vec<Content>>,
}

#[allow(dead_code)]
impl Compartment {
    pub fn new(
        height: f64,
        width: f64,
        depth: f64,
        backwall: Option<f64>,
        content: Option<Vec<Content>>,
    ) -> Self {
        Compartment {
            height,
            width,
            depth,
            backwall,
            content,
        }
    }
}
