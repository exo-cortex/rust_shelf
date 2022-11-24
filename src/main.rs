// use std::fs::File;

// #[allow(dead_code)]
mod board;
// mod compartment;
mod shelf;

use shelf::Depth;

fn main() {
    let mut my_shelf = shelf::Shelf::new(2500.0, 2500.0, 400.0, 550.0, 18.0);

    // my_shelf.add_compartments(&[320.0]);
    my_shelf
        .add_level(250.0)
        .add_compartments(&[350.0], Depth::Extra)
        .add_compartments(&[300.0, 250.0, 150.0, 350.0, 400.0, 250.0], Depth::Normal)
        .extend_to_full_width(Depth::Extra);
    my_shelf
        .add_level(120.0)
        .add_compartments(&[110.0, 400.0, 250.0], Depth::Normal)
        .extend_to_full_width(Depth::Normal);
    my_shelf
        .add_level(350.0)
        .add_compartments(&[350.0], Depth::Normal)
        .add_compartments(&[150.0, 350.0, 550.0], Depth::Extra)
        .extend_to_full_width(Depth::Normal);
    my_shelf
        .add_level(350.0)
        .add_compartments(&[350.0], Depth::Normal)
        .add_compartments(&[150.0, 350.0, 550.0], Depth::Extra)
        .extend_to_full_width(Depth::Normal);
    // my_shelf.randomize_depths();

    println!("{}", my_shelf);
}
