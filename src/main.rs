// use std::fs::File;

// #[allow(dead_code)]
mod board;
mod shelf;

fn main() {

	let mut my_shelf = shelf::Shelf::new(3500.0, 2500.0, 400.0, 550.0, 18.0);

	my_shelf.add_level(350.0)
		.add_compartments(&[100.0])
		.deep().add_compartments(&[300.0])
		.normal().extend_to_full_width();
	my_shelf.add_level(120.0).add_compartments(&[110.0, 400.0, 250.0])
		.deep().extend_to_full_width();
	my_shelf.add_level(350.0).deep().add_compartments(&[350.0]).normal().add_compartments(&[150.0, 350.0, 550.0])
		.normal().extend_to_full_width();
	println!("{}", my_shelf);
}
