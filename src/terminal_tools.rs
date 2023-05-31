use std::io::{stdout, Write};

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Colour {
    RESET = 0,
    BLACK = 30,
    RED,
    GREEN,
    YELLOW,
    BLUE,
    MAGENTA,
    CYAN,
    WHITE
}

pub fn jump(x: usize, y: usize) {
    print!("\x1b[{};{}H", x+1, y+1);
}

pub fn newline() {
    print!("\x1b[1E");
}

pub fn put_character(c: char) {
    print!("{c}")
}

fn apply_esc_seq(code: u8) {
    print!("\x1b[1;{}m", code);
}

pub fn set_colour(colour: Colour) {
    apply_esc_seq(colour as u8);
}

fn as_bg_colour(colour: Colour) -> u8 {
    if colour == Colour::RESET {
        0
    } else {
        colour as u8 + 10
    }
}

pub fn set_bg_colour(colour: Colour) {
    apply_esc_seq(as_bg_colour(colour));
}

pub fn flush() {
    stdout().flush().unwrap();
}
