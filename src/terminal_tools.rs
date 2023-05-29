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

pub fn put_character(c: char) {
    println!("{c}{c}")
}

fn apply_esc_seq(code: u8) {
    println!("\x1b[1;{}m", code);
}

pub fn set_colour(colour: Colour) {
    apply_esc_seq(colour as u8);
}

fn as_bg_colour(colour: Colour) -> u8 {
    if colour == Colour::RESET {
        return 0;
    } else {
        return colour as u8 + 10;
    }
}

pub fn set_bg_colour(colour: Colour) {
    apply_esc_seq(as_bg_colour(colour));
}

pub fn flush() {
    stdout().flush();
}
