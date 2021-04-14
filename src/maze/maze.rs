use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt;

use super::{square::Square};
use super::directions::Direction;


pub struct Maze {
    maze: Vec<Vec<Square>>,
    cols: usize,
    rows: usize,
}

impl Maze {
    pub fn new(rows: usize, cols: usize) -> Maze {
        let mut m = Vec::with_capacity(rows);
        for row in 0..rows {
            m.push(Vec::with_capacity(cols));
            for col in 0..cols {
                m[row].push(Square::new(row, col));
            }
        }

        Maze {
            maze: m,
            cols: cols,
            rows: rows,
        }
    }

    fn get_neighbour(&self, row: usize, col: usize, dir: &Direction) -> Option<&Square> {
        // Checking if square exists
        if row >= self.rows || col >= self.cols {
            return None;
        }

        match dir {
            Direction::Up    => self.maze.get(row - 1)?.get(col),
            Direction::Down  => self.maze.get(row + 1)?.get(col),
            Direction::Left  => self.maze.get(row)?.get(col - 1),
            Direction::Right => self.maze.get(row)?.get(col + 1),
        }
    }

    fn get_neighbours(&self, row: usize, col: usize) -> Vec<&Square> {
        let mut neighbours = vec![];
        for dir in Direction::all() {
            if let Some(neighbour) = self.get_neighbour(row, col, &dir) {
                neighbours.push(neighbour);
            }
        }
        neighbours
    }

    // fn get_neighbours_mut isn't possible because of multiple &mut
    fn get_neighbour_mut(&mut self, row: usize, col: usize, dir: &Direction) -> Option<&mut Square> {
        // Checking if square exists
        if row >= self.rows || col >= self.cols {
            return None;
        }

        match dir {
            Direction::Up    => self.maze.get_mut(row - 1)?.get_mut(col),
            Direction::Down  => self.maze.get_mut(row + 1)?.get_mut(col),
            Direction::Left  => self.maze.get_mut(row)?.get_mut(col - 1),
            Direction::Right => self.maze.get_mut(row)?.get_mut(col + 1),
        }
    }

    pub fn has_path(&self, row: usize, col: usize, dir: &Direction) -> Option<bool> {
        let square = self.maze.get(col)?.get(row)?;
        Some(square.dirs.has_dir(dir))
    }

    // None if can't add path (Maze border)
    // Some(false) if path already exists
    // Some(true)  if path is added
    pub fn remove_wall(&mut self, row: usize, col: usize, dir: &Direction) -> Option<bool> {

        // Returns if either square or neighbour doesn't exist
        let neighbour = self.get_neighbour_mut(row, col, dir)?;
        neighbour.dirs.add_dir(&dir.get_opp());

        let square = self.maze.get_mut(row)?.get_mut(col)?;
        Some( square.dirs.add_dir(dir) )
    }

    pub fn add_wall(&mut self, row: usize, col: usize, dir: &Direction) -> Option<bool> {

        // Returns None if either square or neighbour doesn't exist
        let neighbour = self.get_neighbour_mut(row, col, dir)?;
        neighbour.dirs.remove_dir(&dir.get_opp());

        let square = self.maze.get_mut(row)?.get_mut(col)?;
        Some( square.dirs.remove_dir(dir) )
    }


    // Checks if there is a path from start (0,0) to finish (height-1, width-1)
    // Just goes to all possible squares
    pub fn is_solvable(&self) -> bool {
        // Points already checked
        let mut checked = HashSet::new();

        // Stack of points to check
        let mut stack = VecDeque::new();
        stack.push_front((0, 0));

        while let Some((row, col)) = stack.pop_front() {

            for dir in self.maze[row][col].dirs.available() {
                let neighbour = self.get_neighbour(row, col, &dir).unwrap();
                
                // println!("{} {} {:?} ({}, {})", row, col, &dir, neighbour.get_row(), neighbour.get_col());

                if !checked.contains(&(neighbour.get_row(), neighbour.get_col())) {
                    stack.push_front((neighbour.get_row(), neighbour.get_col()));
                }

                if (neighbour.get_row(), neighbour.get_col()) == (self.rows - 1, self.cols - 1) {
                    return true;
                }
            }

            checked.insert((row, col));
        }

        false
    }


    // Solves maze and sets Square.is_path, returns false if not solvable
    // Just goes to all possible squares
    pub fn solve(&mut self) -> bool {
        // key: (row, col) of Square already checked
        // value: Where did we come from to end up on this square
        //        eg: checked (0,0) then moved on to (1,0)
        //            checked.insert((1,0), Some(Direction::Up))
        //              Option because no direction at starting point
        let mut checked = HashMap::new();

        // Stack of points to check
        let mut stack = VecDeque::new();
        stack.push_front((0, 0, None));

        let mut solvable = false;
        while let Some((row, col, came_from)) = stack.pop_front() {

            // dir.available returns all directions without walls
            for dir in self.maze[row][col].dirs.available() {
                let neighbour = self.get_neighbour(row, col, &dir).unwrap();
                
                // println!("{} {} {:?} ({}, {})", row, col, &dir, neighbour.get_row(), neighbour.get_col());

                // If we haven't seen this square before, add it to the stack
                if !checked.contains_key(&(neighbour.get_row(), neighbour.get_col())) {
                    stack.push_front((neighbour.get_row(), neighbour.get_col(), Some(dir.get_opp())));
                }

                // If this square is the finish point, we're done and the maze is solved
                if (neighbour.get_row(), neighbour.get_col()) == (self.rows - 1, self.cols - 1) {
                    solvable = true;
                    break;
                }
            }

            checked.insert((row, col), came_from);
        }

        if !solvable {
            return false;
        }

        let mut i = self.rows - 1;
        let mut j = self.cols - 1;

        // Walk backwards from the finish line because checked remembers where we came from
        while let Some(dir_option) =  checked.get(&(i, j)) {
            self.maze[i][j].set_is_path(true);
            match dir_option {
                Some(dir) => {
                    let neighbour = self.get_neighbour(i, j, dir).unwrap();
                    i = neighbour.get_row();
                    j = neighbour.get_col();
                },
                None => {
                    // We have walked all the way back to the starting point
                    // assert_eq!((i, j), (0, 0));
                    break;
                }
            }
        }

        true
    }

    /*
    Converts maze to a string (with path if is_path is set), returns Vec of the lines
    Example with path:
    +---+---+---+---+---+---+---+---+---+---+
    | * |   |   |   |   |   |   |   |   |   |
    +   +---+---+---+---+---+---+---+---+---+
    | *                                     |
    +   +---+---+---+---+---+   +---+---+---+
    | * |   |   |   |   |   |   |   |   |   |
    +   +---+---+---+---+---+   +---+---+---+
    | *   *   *   *   *   *   *             |
    +   +---+---+---+---+---+   +---+---+---+
    |   |   |   |   |   |   | * |   |   |   |
    +   +---+---+---+---+---+   +---+---+---+
    |   |   |   |   |   |   | *   *   *   * |
    +---+---+---+---+---+---+---+---+---+---+
    */
    pub fn to_lines(&self) -> Vec<String> {
        let mut lines = Vec::with_capacity(2 * self.rows + 1);
        for row in 0..self.rows {
            
            // The +---+   +---+ line
            let mut border_up = String::with_capacity(4 * self.cols + 1);

            // The |   |       | line
            let mut content_line = String::with_capacity(4 * self.cols + 1);
            for col in 0..self.cols {
                let square = &self.maze[row][col];

                border_up += {
                    if square.dirs.has_dir(&Direction::Up) {
                        "+   "
                    } else {
                        "+---"
                    }
                };

                content_line += {
                    if square.dirs.has_dir(&Direction::Left) {
                        " "
                    } else {
                        "|"
                    }
                };
                content_line += {
                    if square.get_is_path() {
                        " * "
                    } else {
                        "   "
                    }
                };
            }
            border_up += "+";
            content_line += "|";

            lines.push(border_up);
            lines.push(content_line);
        }

        let mut last_line = String::new();

        for _ in 0..self.cols {
            last_line += "+---";
        }

        lines.push(last_line + "+");

        lines
    }

    // basically self.to_lines().join('\n')
    pub fn to_string(&self) -> String {
        let mut out = String::with_capacity((4 * self.cols + 2) * (2 * self.rows + 1));
        for line in self.to_lines() {
            out += &line;
            out += "\n";
        }

        out
    }
}

impl fmt::Display for Maze {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}