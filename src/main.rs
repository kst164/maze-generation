use std::env;

mod maze;
use maze::Direction;
use maze::Maze;
use maze::RandomWalls;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("usage: cargo r[un] columns rows");
        println!("Invalid number of arguments");
        return;
    }

    let rows = if let Ok(n) = args[1].parse() {
        n
    } else {
        println!("usage: cargo r[un] rows columns");
        println!("columns is not a number");
        return;
    };
    let cols = if let Ok(n) = args[2].parse() {
        n
    } else {
        println!("usage: cargo r[un] rows columns");
        println!("rows is not a number");
        return;
    };

    //run(rows, cols);

    let mut generator = RandomWalls::new();
    let (m2, attempts) = generator.new_maze(rows, cols);
    println!("{}", m2);
    println!("{}", attempts);
}

fn run(rows: usize, cols: usize) {
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

    println!("{}", m);
    m.toggle_wall(6, 6, Direction::Down);
    println!("{}", m);
}
