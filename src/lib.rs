use wasm_bindgen::prelude::*;

mod maze;
pub use maze::{Direction, Maze, RandomWalls};

#[wasm_bindgen]
pub fn test_maze(rows: usize, cols: usize) -> Maze {
    let mut m = Maze::new(rows, cols);
    // m.remove_wall(1, 2, Direction::Right);
    // m.remove_wall(1, 2, Direction::Up);
    // m.remove_wall(1, 2, Direction::Down);
    // m.remove_wall(1, 2, Direction::Left);

    let i = 4;
    let j = 6;

    for p in 0..rows {
        m.remove_wall(p, 0, Direction::Down);
        if p < i {
            m.remove_wall(p, 0, Direction::Down);
        } else {
            m.remove_wall(p, j, Direction::Down);
        }
    }

    for q in 0..cols {
        m.remove_wall(i, q, Direction::Right);
        m.remove_wall(6, q, Direction::Right);
        if q < j {
            m.remove_wall(i, q, Direction::Right);
        } else {
            m.remove_wall(rows - 1, q, Direction::Right);
        }
    }

    m
}

#[wasm_bindgen]
pub fn get_maze(rows: usize, cols: usize) -> Maze {
    Maze::new(rows, cols)
}
