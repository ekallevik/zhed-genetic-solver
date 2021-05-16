use cell::{GameCell, CellType};

use crate::board::board::Board;
use crate::board::row::Row;
use crate::utils::{Direction, Position};

mod board;
mod cell;
mod utils;

fn main() {
    // fixme: implement bounds check!
    let mut board =  Board::new(
        vec![
            vec![0, 0, 1],
            vec![1, 1, 0],
            vec![0, 0, 10]
        ],
        Position::new(2, 2)
    );

    //fixme: double-move bug
    board.apply_move(Position::new(1, 1), Direction::Right);
    board.apply_move(Position::new(1, 0), Direction::Right);
    board.apply_move(Position::new(0, 0), Direction::Right);
    board.apply_move(Position::new(0, 2), Direction::Down);

    match board.is_solved() {
        true => println!("Solved 🚀"),
        false => println!("Game over")
    }
    println!("Score: {}", board.get_score());
}
