extern crate ncurses;

use ncurses::*;
use crate::visuals::VisualsRegistry;
use std::sync::atomic::{AtomicI32, Ordering};
use std::ops::Drop;

pub struct NcursesContext {
    visuals: VisualsRegistry,
}

impl NcursesContext {
    pub fn get_visuals(&self) -> &VisualsRegistry {
        &self.visuals
    }

    pub fn get_last_pressed_key(&self) -> Option<i32> {
        ncurses::nodelay(ncurses::stdscr(), true);
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

    pub fn get_key(&self) -> i32 {
        ncurses::nodelay(ncurses::stdscr(), false);
        ncurses::getch()
    }

    pub fn clear_key_queue(&self) {
        ncurses::nodelay(ncurses::stdscr(), true);        
        while ncurses::getch() != ncurses::ERR {}
    }
}

pub fn initialize() -> Option<NcursesContext> {
    initscr();
    start_color();
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    refresh();
    Some(NcursesContext{
        visuals: VisualsRegistry::build()?
    })
}

pub fn cleanup() {
    endwin();
}

impl Drop for NcursesContext {
    fn drop(&mut self) {
        cleanup();
    }
}


static _NEXT_FREE_COLOR_ID: AtomicI32 = AtomicI32::new(1);

#[derive(Copy, Clone)]
pub struct ColorPair {
    id: i16
}

pub struct Dimensions {
    x: i32,
    y: i32,
    x_size: i32,
    y_size: i32,
}

impl Dimensions {
    pub fn new(x: i32, y: i32, x_size: i32, y_size: i32) -> Dimensions {
        Dimensions{x, y, x_size, y_size}
    }
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

impl std::ops::BitOr<Attributes> for Attributes {

    type Output = Attributes;

    fn bitor(self, rhs: Self) -> Self::Output {
        Attributes{value: self.value | rhs.value}
    }
}

impl std::ops::BitOr<u64> for Attributes {

    type Output = Attributes;

    fn bitor(self, rhs: u64) -> Self::Output {
        Attributes{value: self.value | rhs}
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

pub struct BasicWindow {
    win: WINDOW
}

impl BasicWindow {

    pub fn new(Dimensions { x, y, x_size, y_size }: Dimensions) -> BasicWindow {
        let w = BasicWindow { win: newwin(x_size, y_size, x, y) };
        w
    }

    pub fn refresh(&self) {
        wrefresh(self.win);
    }

    pub fn move_cur(&self, x: i32, y: i32) {
        wmove(self.win, x, y);
    }

    pub fn print(&self, s: &str) {
        waddstr(self.win, s);
    }

    pub fn move_print(&self, x: i32, y: i32, s: &str) {
        mvwaddstr(self.win, x, y, s);
    }

    pub fn put_character(&self, ch: PrintableCharacter) {
        waddch(self.win, ch.value);
    }

    pub fn move_put(&self, x: i32, y: i32, ch: PrintableCharacter) {
        self.move_cur(x, y);
        self.put_character(ch);
    }

    pub fn apply_attr(&self, attr: Attributes) {
        wattr_on(self.win, attr.value);
    }

    pub fn disable_attr(&self, attr: Attributes) {
        wattr_off(self.win, attr.value);
    }

    pub fn clear_attr(&self) {
        wattrset(self.win, A_NORMAL());
    }

    pub fn set_attr(&self, attr: Attributes) {
        wattrset(self.win, attr.value);
    }

}

impl Drop for BasicWindow {
    fn drop(&mut self) {
        delwin(self.win);
    }
}

pub trait Window : {
    fn refresh(&self);
    fn draw(&self);
    fn handle_keypress(&mut self, key: i32);
}
