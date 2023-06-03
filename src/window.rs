extern crate ncurses;

use ncurses::*;

pub fn initialize() {
    initscr();
    refresh();
}

pub fn cleanup() {
    endwin();
}

struct Window {
    win: WINDOW
}

impl Window {

    pub fn new(x: i32, y:i32, x_size: i32, y_size: i32) -> Window {
        let w = Window { win: newwin(y_size, x_size, y, x) };
        w
    }

    pub fn refresh(&self) {
        wrefresh(self.win);
    }

    pub fn move_cur(&self, x: i32, y: i32) {
        wmove(self.win, y, x);
    }

    pub fn print(&self, s: &str) {
        wprintw(self.win, s);
    }

    pub fn move_print(&self, x: i32, y: i32, s: &str) {
        self.move_cur(x, y);
        self.print(s);
    }

}

impl Drop for Window {
    fn drop(&mut self) {
        delwin(self.win);
    }
}
