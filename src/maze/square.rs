use super::directions::Directions;

#[derive(Debug)]
pub struct Square {
    pub dirs: Directions,
}

impl Square {
    pub fn new() -> Square {
        Square {
            dirs: Directions::new(),
        }
    }
}
