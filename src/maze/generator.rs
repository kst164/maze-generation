use super::maze::Maze;

pub struct MazeGenerator {
    mazes: Vec<Maze>,
}

impl MazeGenerator {
    pub fn new() -> Self {
        Self {
            mazes: vec![],
        }
    }

    pub fn get_maze(&self, n: usize) -> Option<&Maze> {
        self.mazes.get(n)
    }

    pub fn new_maze(&mut self, width: usize, height: usize) -> &Maze {
        let mut maze = Maze::new(width, height);

        // TODO

        self.mazes.push(maze);

        self.mazes.last().unwrap()
    }
}
