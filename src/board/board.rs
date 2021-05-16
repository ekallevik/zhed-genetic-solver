use crate::board::row::Row;
use std::fmt;
use crate::cell::{CellType, GameCell};
use crate::utils::{Direction, Position};

pub struct Board {
    pub size: usize,
    pub rows: Vec<Row>,
    pub goal: Position,
}

impl Board {

    pub(crate) fn new(row_values: Vec<Vec<u8>>, goal: Position) -> Board {

        let size = row_values[0].len();

        let rows = row_values
            .into_iter()
            .map(|values| {
                assert_eq!(values.len(), size);
                Row::new(values)
            })
            .collect::<Vec<Row>>();

        let board = Board{
            size,
            rows,
            goal,
        };

        assert!(board.get_cell(goal).cell_type==CellType::Goal, "Misplaced goal");
        board
    }

    pub(crate) fn apply_move(&mut self, position: Position, direction: Direction) {
        let mut value = self.grow_from(position);
        let mut next = Board::get_next_position(position, direction);

        while value > 0 && next.is_valid(self.size) {
            let grown = self.grow(next);
            if grown {
                value -= 1;
            }
            next = Board::get_next_position(next, direction);
        }
        println!("{}", self)
    }

    fn grow_from(&mut self, position: Position) -> u8 {
        let Position { row: x, col: y } = position;

        match self.rows[x].cells[y].cell_type {
            CellType::Source => {
                self.rows[x].cells[y].grown = true;
                self.rows[x].cells[y].value
            }
            _ => 0
        }
    }

    fn grow(&mut self, position: Position) -> bool {
        let Position { row: x, col: y } = position;
        self.rows[x].grow(y)
    }

    fn get_next_position(position: Position, direction: Direction) -> Position {
        let Position { row: x, col: y } = position;

        match direction {
            Direction::Up => { Position { row: x - 1, col: y } }
            Direction::Right => { Position { row: x, col: y + 1 } }
            Direction::Down => { Position { row: x + 1, col: y } }
            Direction::Left => { Position { row: x, col: y - 1 } }
        }
    }

    pub fn get_score(&self) -> usize {
        match self.is_solved() {
            // todo: add a proper way to calculate score. Remember to take board size into account
            true => 1000,
            false => self.calculate_score(),
        }
    }

    // todo: add docs and scoring function
    fn calculate_score(&self) -> usize {

        let mut score: usize = 0;

        for i in 0..self.size {
            for j in 0..self.size {
                let current_pos = Position::new(i, j);
                let current_cell = self.get_cell(current_pos);

                if current_pos.is_valid(self.size) && current_cell.grown {
                    let distance = Board::calculate_distance(current_pos, self.goal);
                    score += match distance {
                        1 => 100,
                        2 => 30,
                        3 => 10,
                        _ => 0,
                    }
                }
            }
        }

        score
    }

    pub fn calculate_distance(a: Position, b: Position) -> usize {
        (
            (a.row as isize - b.row as isize).abs() + (a.col as isize - b.col as isize).abs()
        ) as usize
    }

    pub fn is_solved(&self) -> bool {
        self.get_goal_cell().grown
    }

    fn get_goal_cell(&self) -> &GameCell {
        self.get_cell(self.goal)
    }

    fn get_cell(&self, position: Position) -> &GameCell {
        &self.rows[position.row].cells[position.col]
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Score: {}", self.get_score());
        for row in &self.rows {
            write!(f, "{}\n", row);
        }
        Ok(())
    }
}
