use std::cell::RefCell;
use std::rc::Rc;

use crate::board::*;
use crate::basic_window::*;

pub struct VisualsRegistry {
    pub common_visuals: Rc<CommonVisuals>,
    pub snake_visuals: RefCell<SnakeVisuals>,
}

impl VisualsRegistry {

    pub fn build() -> Option<VisualsRegistry> {
        let common_visuals = Rc::new(CommonVisuals::build()?.into());
        Some(VisualsRegistry{
            snake_visuals: SnakeVisuals::build(Rc::clone(&common_visuals))?.into(),
            common_visuals
        })
    }
}

pub struct CommonVisuals {
    pub basic_colors: Vec<ColorPair>
}

impl CommonVisuals {

    fn make_basic_colors() -> Option<Vec<ColorPair>> {
        (ncurses::COLOR_BLACK..ncurses::COLOR_WHITE + 1).map(|c| ColorPair::new(c, ncurses::COLOR_BLACK)).collect()
    }

    pub fn get_colour(&self, c: i16) -> ColorPair {
        self.basic_colors[c as usize]
    }

    pub fn build() -> Option<CommonVisuals> {
        Some(CommonVisuals{
            basic_colors: CommonVisuals::make_basic_colors()?,
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
    pub common_visuals: Rc<CommonVisuals>,
}

impl SnakeVisuals {
    fn build(common: Rc<CommonVisuals>) -> Option<SnakeVisuals> {
        Some(SnakeVisuals {
            none:       PrintableCharacter::new(' ', ColorPair::new(ncurses::COLOR_BLACK, ncurses::COLOR_BLACK)?.into()),
            wall:       PrintableCharacter::new(' ', ColorPair::new(ncurses::COLOR_BLACK, ncurses::COLOR_BLUE )?.into()),
            food:       PrintableCharacter::new('*', ColorPair::new(ncurses::COLOR_GREEN, ncurses::COLOR_BLACK)?.into()),
            colors_head: ColorPair::new(ncurses::COLOR_RED,    ncurses::COLOR_BLACK)?,
            colors_body: ColorPair::new(ncurses::COLOR_YELLOW, ncurses::COLOR_BLACK)?,
            colors_points: ColorPair::new(ncurses::COLOR_MAGENTA, ncurses::COLOR_WHITE)?,
            colors_ending: ColorPair::new(ncurses::COLOR_CYAN, ncurses::COLOR_WHITE)?,
            common_visuals: common
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

    pub fn get(&self, category: &BrickType) -> PrintableCharacter {
        match category {
            BrickType::None => self.none,
            BrickType::Wall => self.wall,
            BrickType::Snake(dir) => PrintableCharacter::new(Self::get_char_from_direction(*dir), self.colors_body.into()),
            BrickType::SnakeHead(dir) => PrintableCharacter::new(Self::get_char_from_direction(*dir), self.colors_head.into()),
            BrickType::Food => self.food,
            BrickType::Portal(data) => PrintableCharacter::new('@', self.common_visuals.get_colour(data.colour).into())
        }
    }
}