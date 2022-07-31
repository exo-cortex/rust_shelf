// use std::fs::File;

// #[allow(dead_code)]
mod board;
mod shelf;

fn main() {
    let mut board = board::Board {
        width: 320.0,
        height: 150.0,
        thickness: 18.0,
        holes: Vec::new(),
    };

    board.drill(board::DrillSide::Left(-20.0), 10.0, 20.0);
    board.drill(board::DrillSide::Left(20.0), 10.0, 20.0);
    board.drill(board::DrillSide::Right(-20.0), 10.0, 20.0);
    board.drill(board::DrillSide::Right(20.0), 10.0, 20.0);
}
