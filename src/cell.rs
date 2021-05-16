use std::fmt;

#[derive(PartialEq)]
pub enum CellType {
    Source,
    Growable,
    Goal,
}

pub struct GameCell {
    pub(crate) cell_type: CellType,
    pub value: u8,
    pub grown: bool,
}

impl GameCell {

    pub fn new(value: u8) -> GameCell {

        // fixme: implement a solution that does not rely on hardcoding
        match value {
            0 => GameCell{cell_type: CellType::Growable, value, grown: false},
            1..=9 => GameCell{cell_type: CellType::Source, value, grown: false},
            _ => GameCell{cell_type: CellType::Goal, value: 0, grown: false }
        }
    }

    pub fn is_growable(&self) -> bool {
        !self.grown && self.cell_type != CellType::Source
    }

    pub fn grow(&mut self) -> bool {
        match self.is_growable() {
            false => false,
            true => {
                self.grown = true;
                true
            }
        }
    }
}

impl fmt::Display for GameCell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        match self.cell_type {
            CellType::Source => {
                write!(f, "{}", if self.grown {"X".to_owned()} else {self.value.to_string()})
            }
            CellType::Growable => {
                write!(f, "{}", if self.grown {"x"} else {"."})
            }
            CellType::Goal => {
                write!(f, "{}", if self.grown {"Ø"} else {"*"})
            }
        }
    }
}
