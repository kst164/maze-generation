use super::directions::Directions;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Square {
    pub(super) dirs: Directions,
    row: usize,
    col: usize,
    is_path: bool,
}

impl Square {
    pub fn new(row: usize, col: usize) -> Square {
        Square {
            dirs: Directions::new(),
            row: row,
            col: col,
            is_path: false,
        }
    }

    pub fn get_row(&self) -> usize {
        self.row
    }

    pub fn get_col(&self) -> usize {
        self.col
    }

    pub fn get_is_path(&self) -> bool {
        self.is_path
    }

    pub fn set_is_path(&mut self, b: bool) {
        self.is_path = b;
    }
}
