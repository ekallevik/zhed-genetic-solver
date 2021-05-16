use std::fmt;

use crate::cell::GameCell;

pub struct Row {
    pub(crate) cells: Vec<GameCell>,
}

impl Row {

    pub(crate) fn new(values: Vec<u8>) -> Row {
        
        let mut res: Vec<GameCell> = vec![];
        for value in values {
            res.push(GameCell::new(value));
        }

        Row{ cells: res }
    }

    pub(crate) fn grow(&mut self, index: usize) -> bool {
        self.cells[index].grow()
    }
}

impl fmt::Display for Row {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for cell in &self.cells {
            write!(f, "{} ", cell);
        }
        Ok(()) // todo: add Ok(()) everywhere
    }
}

