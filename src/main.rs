mod snake_window;
mod window;
mod board_file;

extern crate ncurses;

use std::{thread, time};
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
    window::initialize();
    ncurses::keypad(ncurses::stdscr(), true);
    let wait_time = time::Duration::from_millis(300);

    let game_window = SnakeWindow::new_default(15, 30);
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
            match key.unwrap() {
                ncurses::KEY_UP    => { game_window.turn(snake_window::Direction::Up);    }
                ncurses::KEY_RIGHT => { game_window.turn(snake_window::Direction::Right); }
                ncurses::KEY_DOWN  => { game_window.turn(snake_window::Direction::Down);  }
                ncurses::KEY_LEFT  => { game_window.turn(snake_window::Direction::Left);  }
                _ => {}
            }
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
    window::cleanup();
}
