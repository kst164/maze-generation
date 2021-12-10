extern crate rand;
use rand::prelude::*;

use super::Direction;
use super::Maze;

pub struct RandomWalls {
    mazes: Vec<Maze>,
}

impl RandomWalls {
    pub fn new() -> Self {
        Self { mazes: vec![] }
    }

    pub fn get_maze(&self, n: usize) -> Option<&Maze> {
        self.mazes.get(n)
    }

    pub fn new_maze(&mut self, rows: usize, cols: usize) -> (&Maze, u32) {
        let mut maze = Maze::new(rows, cols);

        // Actually a couple extra, but whatever
        let mut walls: Vec<u8> = vec![0; 2 * rows * cols];

        let mut rng = thread_rng();

        let mut attempts = 0;

        while !maze.solve() {
            attempts += 1;
            println!("Making random array");
            rng.fill(walls.as_mut_slice());
            println!("Made random array");

            let mut bools = walls.iter();

            const MIN_TO_REMOVE: u8 = 0xAF;

            for row in 0..rows {
                for col in 0..cols {
                    if *bools.next().unwrap() > MIN_TO_REMOVE {
                        maze.remove_wall(row, col, Direction::Right);
                    }
                    if *bools.next().unwrap() > MIN_TO_REMOVE {
                        maze.remove_wall(row, col, Direction::Down);
                    }
                }
            }
        }

        self.mazes.push(maze);

        (self.mazes.last().unwrap(), attempts)
    }
}
