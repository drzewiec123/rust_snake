use crate::terminal_tools as TL;
use std::vec;

#[derive(Copy, Clone, PartialEq, Eq)]
enum BrickType {
    NONE,
    WALL,
    SNAKE,
    FOOD
}

#[derive(Copy, Clone, PartialEq, Eq)]
struct BrickVisuals {
    character: char,
    fg_colour: TL::Colour,
    bg_colour: TL::Colour
}

impl BrickVisuals {
    pub fn apply(&self) {
        TL::set_colour(self.fg_colour);
        TL::set_bg_colour(self.bg_colour);
    }
}

fn get_brick_visuals(category: BrickType) -> BrickVisuals {
    match category {
        BrickType::NONE  => BrickVisuals { character: ' ', fg_colour: TL::Colour::BLACK, bg_colour: TL::Colour::BLACK },
        BrickType::WALL  => BrickVisuals { character: '█', fg_colour: TL::Colour::BLUE,  bg_colour: TL::Colour::BLACK },
        BrickType::SNAKE => BrickVisuals { character: '%', fg_colour: TL::Colour::RED,   bg_colour: TL::Colour::BLACK },
        BrickType::FOOD  => BrickVisuals { character: '*', fg_colour: TL::Colour::GREEN, bg_colour: TL::Colour::BLACK },
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Direction {
    UP = 0,
    RIGHT,
    DOWN,
    LEFT
}

#[derive(Copy, Clone)]
struct Position(usize, usize);

impl Position {
    pub fn move_dir(&self, dir: Direction) -> Position {
        let Position(x, y) = *self;
        match dir {
            Direction::DOWN  => Position(x + 1, y),
            Direction::LEFT  => Position(x, y - 1),
            Direction::UP    => Position(x - 1, y),
            Direction::RIGHT => Position(x, y + 1)
        }
    }
}
pub struct SnakeWindow {
    x_size: usize,
    y_size: usize,
    head_pos: Position,
    facing: Direction,
    last_step: Direction,
    board: Vec::<Vec::<BrickType>>
}

impl SnakeWindow {

    fn create_board(x_size: usize, y_size: usize) -> Vec::<Vec::<BrickType>> {
        let mut board = vec![vec![BrickType::WALL; y_size]; x_size];
        for i in 1..x_size - 1 {
            for j in 1..y_size - 1 {
                board[i][j] = BrickType::NONE;
            }
        }
        board
    }

    fn place_snake(&mut self) {
        for x in self.head_pos.0..self.head_pos.0 + 4 {
            self.board[x][self.head_pos.1] = BrickType::SNAKE
        }
    }

    pub fn new(x_size: usize, y_size: usize) -> Self {
        let mut sw = SnakeWindow {
            x_size: x_size,
            y_size: y_size,
            head_pos: Position(x_size / 2, y_size / 2),
            facing: Direction::UP,
            last_step: Direction::UP,
            board: Self::create_board(x_size, y_size)
        };
        sw.place_snake();
        println!("created pos {} {}", sw.head_pos.0, sw.head_pos.1);
        sw
    }

    fn end_drawing_session(&self) {
        TL::jump(0, self.y_size);
        TL::flush();
    }

    fn change_brick(&mut self, Position(x, y): Position, category: BrickType) {
        self.board[x][y] = category;
        let visuals = get_brick_visuals(category);
        visuals.apply();
        TL::jump(x, y);
        TL::put_character(visuals.character);
    }

    pub fn draw_board(&self) {
        let mut visual = get_brick_visuals(self.board[0][0]);
        visual.apply();
        TL::jump(0, 0);
        for row in &self.board {
            for brick in row {
                let new_visual = get_brick_visuals(*brick);
                if new_visual != visual {
                    visual = new_visual;
                    visual.apply();
                }
                TL::put_character(visual.character);
            }
            TL::newline();
        }
        self.end_drawing_session();
    }

    pub fn turn(&mut self, dir: Direction) {
        if (dir as u8) != (self.last_step as u8 + 2) % 4 {
            self.facing = dir;
        }
    }

    pub fn step(&mut self) -> bool {
        self.last_step = self.facing;
        let new_pos = self.head_pos.move_dir(self.last_step);
        if self.board[new_pos.0][new_pos.1] != BrickType::NONE {
            self.end_drawing_session();
            return false;
        }
        println!("new pos {} {}", new_pos.0, new_pos.1);
        self.change_brick(new_pos, BrickType::SNAKE);
        self.head_pos = new_pos;
        self.end_drawing_session();
        
        true
    }



}

