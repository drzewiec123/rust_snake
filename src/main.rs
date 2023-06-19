mod snake_window;
mod window;
mod board;
mod visuals;
mod board_file;

extern crate ncurses;

use std::{thread, time, env};
use crate::snake_window::SnakeWindow;

fn get_pressed_key() -> Option<i32> {
    let mut last_key = ncurses::ERR;
    loop {
        let key = ncurses::getch();
        if key == ncurses::ERR {
            break;
        }
        last_key = key;
    }
    if last_key == ncurses::ERR {
        None
    } else {
        Some(last_key)
    }
}

fn main() {
    let context = window::initialize();
    if context.is_none() {
        println!("Error: Could not initialize curses");
    }
    let context = context.unwrap();
    ncurses::keypad(ncurses::stdscr(), true);
    let wait_time = time::Duration::from_millis(300);

    let game_window: Option<SnakeWindow>;
    if env::args().len() < 2 {
        game_window = SnakeWindow::new_default(&context, 15, 30);
    } else {
        let board = board_file::from_file(&env::args().nth(1).unwrap());
        if board.is_none() {
            game_window = None;
        } else {
            game_window = SnakeWindow::new(&context, board.unwrap())
        }
    }

    if game_window.is_none() {
        println!("Could not initialize game board :(");
        ncurses::endwin();
        return;
    }

    let mut game_window = game_window.unwrap();
    game_window.draw_board();

    ncurses::getch();
    ncurses::nodelay(ncurses::stdscr(), true);
    ncurses::timeout(0);
    loop {
        thread::sleep(wait_time);
        let mut key: Option<i32>;
        while { key = get_pressed_key(); key.is_some() } {
            game_window.handle_keypress(key.unwrap());
        }
        if !game_window.step() {
            break;
        }
    }
    game_window.draw_ending_message();
    
    while ncurses::getch() != ncurses::ERR {}
    
    ncurses::timeout(-1);
    ncurses::nodelay(ncurses::stdscr(), false);
    ncurses::getch();
}
