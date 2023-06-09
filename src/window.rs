extern crate ncurses;

use ncurses::*;
use std::sync::atomic::{AtomicI32, Ordering};

pub fn initialize() {
    initscr();
    start_color();
    refresh();
}

pub fn cleanup() {
    endwin();
}

pub fn clear_attributes() {
    attrset(A_NORMAL());
}

static _NEXT_FREE_COLOR_ID: AtomicI32 = AtomicI32::new(1);

#[derive(Copy, Clone)]
pub struct ColorPair {
    id: i16
}

impl ColorPair {
    pub fn new(fg: i16, bg: i16) -> Option<ColorPair> {
        let assigned_id = _NEXT_FREE_COLOR_ID.fetch_add(1, Ordering::Relaxed);
        if COLOR_PAIRS() <= assigned_id {
            None
        } else {
            init_pair(assigned_id as i16, fg, bg);
            Some(ColorPair{id: assigned_id as i16})
        }
    }

    pub fn apply(&self) {
        attr_on(COLOR_PAIR(self.id));
    }
}

impl Into<Attributes> for ColorPair {
    fn into(self) -> Attributes {
        Attributes { value: COLOR_PAIR(self.id) }
    }
}

#[derive(Copy, Clone)]
pub struct Attributes {
    value: u64
}

impl Attributes {
    pub fn new(attr: u64) -> Attributes {
        Attributes { value: attr }
    }

    pub fn none() -> Attributes {
        Attributes { value: 0 }
    }
}

impl std::ops::BitOr for Attributes {

    type Output = Attributes;

    fn bitor(self, rhs: Self) -> Self::Output {
        Attributes{value: self.value | rhs.value}
    }
}

#[derive(Copy, Clone)]
pub struct PrintableCharacter {
    value: u64
}

impl PrintableCharacter {
    pub fn new(c: char, attr: Attributes) -> PrintableCharacter {
        PrintableCharacter { value: c as u64 | attr.value}
    }

}

pub struct Window {
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

    pub fn put_character(&self, ch: PrintableCharacter) {
        waddch(self.win, ch.value);
    }

    pub fn move_put(&self, x: i32, y: i32, ch: PrintableCharacter) {
        self.move_cur(x, y);
        self.put_character(ch);
    }

}

impl Drop for Window {
    fn drop(&mut self) {
        delwin(self.win);
    }
}
