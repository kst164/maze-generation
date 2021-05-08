extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up = 0,
    Down = 1,
    Left = 2,
    Right = 3,
}

impl Direction {
    fn as_flag(&self) -> u8 {
        match self {
            Self::Up    => 0b1000,
            Self::Down  => 0b0100,
            Self::Left  => 0b0010,
            Self::Right => 0b0001,
        }
    }

    pub fn all() -> Vec<Self> {
        vec![Self::Up, Self::Right, Self::Down, Self::Left]
    }

    pub fn get_opp(&self) -> Self{
        match self {
            Self::Up    => Self::Down,
            Self::Down  => Self::Up,
            Self::Right => Self::Left,
            Self::Left  => Self::Right,
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Directions(u8);

impl Directions {
    pub fn new() -> Directions {
        Directions(0x0)
    }

    pub fn has_dir(&self, dir: Direction) -> bool {
        self.0 & dir.as_flag() != 0
    }

    // Vec of available directions
    pub fn available(&self) -> Vec<Direction>{
        let mut list = vec![];
        for dir in Direction::all() {
            if self.has_dir(dir) {
                list.push(dir);
            }
        }
        list
    }

    // False if wasn't already there
    pub fn add_dir(&mut self, dir: Direction) -> bool {
        let flag = dir.as_flag();
        if self.0 & flag != 0 {
            false
        } else {
            self.0 |= flag;
            true
        }
    }

    // False if already wasn't there
    pub fn remove_dir(&mut self, dir: Direction) -> bool {
        let flag = dir.as_flag();
        if self.0 & flag == 0 {
            false
        } else {
            self.0 &= !flag;
            true
        }
    }
}
