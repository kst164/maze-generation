use std::env;

extern crate ncurses;

mod maze;
use maze::Maze;
use maze::Direction;

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

    let mut m  = Maze::new(rows, cols);
    // m.remove_wall(1, 2, &Direction::Right);
    // m.remove_wall(1, 2, &Direction::Up);
    // m.remove_wall(1, 2, &Direction::Down);
    // m.remove_wall(1, 2, &Direction::Left);

    let i = 4;
    let j = 6;

    for p in 0..rows {
        m.remove_wall(p, 0, &Direction::Down);
        if p < i {
            m.remove_wall(p, 0, &Direction::Down);
        } else {
            m.remove_wall(p, j, &Direction::Down);
        }
    }

    for q in 0..cols {
        m.remove_wall(i, q, &Direction::Right);
        m.remove_wall(6, q, &Direction::Right);
        if q < j {
            m.remove_wall(i, q, &Direction::Right);
        } else {
            m.remove_wall(rows - 1, q, &Direction::Right);
        }
    }

    // m.add_wall(6, 6, &Direction::Down);

    println!("{}", m);
    println!("{}", m.solve());
    println!("{}", m);
}
