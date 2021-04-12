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

    let length = match args[1].parse() {
        Ok(n) => n,
        Err(_) => {
            println!("usage: cargo r[un] columns rows");
            println!("columns is not a number");
            return;
        }
    };
    let width = match args[2].parse() {
        Ok(n) => n,
        Err(_) => {
            println!("usage: cargo r[un] columns rows");
            println!("rows is not a number");
            return;
        }
    };

    let mut m  = Maze::new(length, width);
    m.add_wall(1, 2, &Direction::Right);
    m.add_wall(1, 2, &Direction::Up);
    m.add_wall(1, 2, &Direction::Down);
    m.add_wall(1, 2, &Direction::Left);

    // ncurses testing, was thinking to use for navigation
    /*ncurses::initscr();
    ncurses::addstr(&m.to_string());
    ncurses::refresh();
    ncurses::getch();
    ncurses::endwin();*/

    println!("{}", m);
}
