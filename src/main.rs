mod snake_window;
mod basic_window;
mod selection_window;
mod board;
mod visuals;
mod board_file;
mod game;
mod event_emitter;

extern crate ncurses;

use game::Game;

fn main() {
    let context = basic_window::initialize();
    if context.is_none() {
        println!("Error: Could not initialize curses");
    }
    let context = context.unwrap();
    ncurses::keypad(ncurses::stdscr(), true);
    let mut game = Game::new(&context);
    game.run();

    
/*
    let mut game_window: SnakeWindow;
    if env::args().len() < 2 {
        game_window = SnakeWindow::new_default(&context, 15, 30);
    } else {
        let board = board_file::from_file(&env::args().nth(1).unwrap());
        if board.is_none() {
            println!("Could not initialize game board :(");
            ncurses::endwin();
            return;
        } else {
            game_window = SnakeWindow::new(&context, board.unwrap())
        }
    }
*/
}
