use std::{collections::{HashSet, VecDeque}, fmt};

use super::square::Square;
use super::directions::Direction;


#[derive(Debug)]
pub struct Maze {
    maze: Vec<Vec<Square>>,
    width: usize,
    height: usize,
}

impl Maze {
    pub fn new(width: usize, height: usize) -> Maze {
        let mut m = Vec::with_capacity(height);
        for y in 0..height {
            m.push(Vec::with_capacity(width));
            for _x in 0..width {
                m[y].push(Square::new());
            }
        }

        Maze {
            maze: m,
            width: width,
            height: height,
        }
    }

    fn get_neighbour(&mut self, row: usize, col: usize, dir: &Direction) -> Option<&mut Square> {
        // Checking if square exists
        if row >= self.height || col >= self.width {
            return None;
        }

        match dir {
            Direction::Up    => self.maze.get_mut(row - 1)?.get_mut(col),
            Direction::Down  => self.maze.get_mut(row + 1)?.get_mut(col),
            Direction::Left  => self.maze.get_mut(row)?.get_mut(col - 1),
            Direction::Right => self.maze.get_mut(row)?.get_mut(col + 1),
        }
    }

    pub fn has_wall(&self, row: usize, col: usize, dir: &Direction) -> Option<bool> {
        let square = self.maze.get(col)?.get(row)?;
        Some(square.dirs.has_dir(dir))
    }

    // None if can't add path (Maze border)
    // Some(false) if path already exists
    // Some(true)  if path is added
    pub fn remove_wall(&mut self, row: usize, col: usize, dir: &Direction) -> Option<bool> {

        // Returns if either square or neighbour doesn't exist
        let neighbour = self.get_neighbour(row, col, dir)?;
        neighbour.dirs.add_dir(&dir.get_opp());

        let square = self.maze.get_mut(row)?.get_mut(col)?;
        Some( square.dirs.add_dir(dir) )
    }

    pub fn add_wall(&mut self, row: usize, col: usize, dir: &Direction) -> Option<bool> {

        // Returns None if either square or neighbour doesn't exist
        let neighbour = self.get_neighbour(row, col, dir)?;
        neighbour.dirs.remove_dir(&dir.get_opp());

        let square = self.maze.get_mut(row)?.get_mut(col)?;
        Some( square.dirs.remove_dir(dir) )
    }

    // Checks if there is a path from start (0,0) to finish (height-1, width-1)
    pub fn is_solvable(&self) -> bool {
        let mut checked = HashSet::new();

        let mut to_check = VecDeque::new();
        to_check.push_front((0, 0));

        while let Some((row, col)) = to_check.pop_front() {
            checked.insert((row, col));

            for dir in self.maze[row][col].dirs.available().iter() {
                
            }
        }

        true
    }

    pub fn to_string(&self) -> String {
        let mut out = String::new();
        for row in 0..self.height {
            for col in 0..self.width {
                out += "+";
                out += if self.maze[row][col].dirs.has_dir(&Direction::Up) {
                    "   "
                } else {
                    "---"
                };
            }

            out += "+\n";

            for col in 0..self.width {
                out += if self.maze[row][col].dirs.has_dir(&Direction::Left) {
                    " "
                } else {
                    "|"
                };

                out += "   ";
            }

            out += "|\n";
        }

        for _col in 0..self.width {
            out += "+---";
        }
        out += "+";

        out
    }
}

impl fmt::Display for Maze {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}