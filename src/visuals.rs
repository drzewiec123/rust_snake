use std::cell::RefCell;

use crate::board::*;
use crate::window::*;

pub struct VisualsRegistry {
    pub snake_visuals: RefCell<SnakeVisuals>,
}

impl VisualsRegistry {
    pub fn build() -> Option<VisualsRegistry> {
        Some(VisualsRegistry{
            snake_visuals: SnakeVisuals::build()?.into()
        })
    }
}

pub struct SnakeVisuals {
    pub none: PrintableCharacter,
    pub wall: PrintableCharacter,
    pub food: PrintableCharacter,
    pub colors_head: ColorPair,
    pub colors_body: ColorPair,
    pub colors_points: ColorPair,
    pub colors_ending: ColorPair,
}

impl SnakeVisuals {
    fn build() -> Option<SnakeVisuals> {
        Some(SnakeVisuals {
            none:       PrintableCharacter::new(' ', ColorPair::new(ncurses::COLOR_BLACK, ncurses::COLOR_BLACK)?.into()),
            wall:       PrintableCharacter::new(' ', ColorPair::new(ncurses::COLOR_BLACK, ncurses::COLOR_BLUE )?.into()),
            food:       PrintableCharacter::new('*', ColorPair::new(ncurses::COLOR_GREEN ,ncurses::COLOR_BLACK)?.into()),
            colors_head: ColorPair::new(ncurses::COLOR_RED,    ncurses::COLOR_BLACK)?,
            colors_body: ColorPair::new(ncurses::COLOR_YELLOW, ncurses::COLOR_BLACK)?,
            colors_points: ColorPair::new(ncurses::COLOR_MAGENTA, ncurses::COLOR_WHITE)?,
            colors_ending: ColorPair::new(ncurses::COLOR_CYAN, ncurses::COLOR_WHITE)?,
        })
    }

    fn get_char_from_direction(dir: Direction) -> char {
        match dir {
            Direction::Up => '^',
            Direction::Right => '>',
            Direction::Down => 'v',
            Direction::Left => '<',
        }
    }

    pub fn get(&self, category: BrickType) -> PrintableCharacter {
        match category {
            BrickType::None => self.none,
            BrickType::Wall => self.wall,
            BrickType::Snake(dir) => PrintableCharacter::new(Self::get_char_from_direction(dir), self.colors_body.into()),
            BrickType::SnakeHead(dir) => PrintableCharacter::new(Self::get_char_from_direction(dir), self.colors_head.into()),
            BrickType::Food => self.food,
        }
    }
}