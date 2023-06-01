use crate::terminal_tools::{self as TL, Colour};
use std::vec;
use std::collections::VecDeque;
use rand::seq::SliceRandom;

#[derive(Copy, Clone, PartialEq, Eq)]
enum BrickType {
    None,
    Wall,
    Snake,
    SnakeHead,
    Food
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

static INITIAL_SIZE: usize = 4;

fn get_brick_visuals(category: BrickType) -> BrickVisuals {
    match category {
        BrickType::None      => BrickVisuals { character: ' ', fg_colour: TL::Colour::Black,  bg_colour: TL::Colour::Black },
        BrickType::Wall      => BrickVisuals { character: '█', fg_colour: TL::Colour::Blue,   bg_colour: TL::Colour::Black },
        BrickType::Snake     => BrickVisuals { character: '%', fg_colour: TL::Colour::Red,    bg_colour: TL::Colour::Black },
        BrickType::SnakeHead => BrickVisuals { character: '%', fg_colour: TL::Colour::Yellow, bg_colour: TL::Colour::Black },
        BrickType::Food      => BrickVisuals { character: '*', fg_colour: TL::Colour::Green,  bg_colour: TL::Colour::Black },
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Direction {
    Up = 0,
    Right,
    Down,
    Left
}

#[derive(Copy, Clone)]
struct Position(usize, usize);

impl Position {
    pub fn move_dir(&self, dir: Direction) -> Position {
        let Position(x, y) = *self;
        match dir {
            Direction::Down  => Position(x + 1, y),
            Direction::Left  => Position(x, y - 1),
            Direction::Up    => Position(x - 1, y),
            Direction::Right => Position(x, y + 1)
        }
    }
}
pub struct SnakeWindow {
    x_size: usize,
    y_size: usize,
    facing: Direction,
    last_step: Direction,
    board: Vec::<Vec::<BrickType>>,
    snake: VecDeque<Position>
}

impl SnakeWindow {

    fn create_board(x_size: usize, y_size: usize) -> Vec::<Vec::<BrickType>> {
        let mut board = vec![vec![BrickType::Wall; y_size]; x_size];
        for i in 1..x_size - 1 {
            for j in 1..y_size - 1 {
                board[i][j] = BrickType::None;
            }
        }
        board
    }

    fn place_snake(&mut self, x: usize, y: usize) {
        for i in (x..x + INITIAL_SIZE).rev() {
            self.board[i][y] = BrickType::Snake;
            self.snake.push_front(Position(i, y));
        }
        self.board[x][y] = BrickType::SnakeHead;
    }

    fn get_head(&self) -> Position {
        *self.snake.front().unwrap()
    }

    pub fn new(x_size: usize, y_size: usize) -> Self {
        let mut sw = SnakeWindow {
            x_size: x_size,
            y_size: y_size,
            facing: Direction::Up,
            last_step: Direction::Up,
            board: Self::create_board(x_size, y_size),
            snake: VecDeque::new()
        };
        sw.place_snake(x_size / 2, y_size / 2);
        let pos = sw.find_valid_food_spawn().unwrap();
        sw.board[pos.0][pos.1] = BrickType::Food;
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
        TL::put_character(self.get_character_for(category, visuals));
    }

    fn get_character_for(&self, brick: BrickType, visuals: BrickVisuals) -> char {
        if brick != BrickType::SnakeHead {
            visuals.character
        } else {
            match self.facing {
                Direction::Up => '^',
                Direction::Right => '>',
                Direction::Down => 'v',
                Direction::Left => '<',
            }
        }
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
                TL::put_character(self.get_character_for(*brick, visual));
            }
            TL::newline();
        }
        self.draw_points();
        self.end_drawing_session();
    }

    pub fn turn(&mut self, dir: Direction) {
        if (dir as u8) != (self.last_step as u8 + 2) % 4 {
            self.facing = dir;
            self.change_brick(self.get_head(), BrickType::SnakeHead);
            self.end_drawing_session();
        }
    }

    fn step_snake(&mut self, new_pos: Position, grow: bool) {
        if !grow {
            let back = self.snake.pop_back().unwrap();
            self.change_brick(back, BrickType::None);
        }

        self.snake.push_front(new_pos);
        self.change_brick(new_pos, BrickType::SnakeHead);
        self.change_brick(self.snake[1], BrickType::Snake);
    }

    fn find_valid_food_spawn(&mut self) -> Option<Position> {
        let mut empty_bricks = Vec::<Position>::new();
        for x in 0..self.board.len() {
            for y in 0..self.board[x].len() {
                if self.board[x][y] == BrickType::None {
                    empty_bricks.push(Position(x, y));
                }
            }
        }
        empty_bricks.choose(&mut rand::thread_rng()).cloned()
    }

    fn spawn_food(&mut self) {
        let pos = self.find_valid_food_spawn();
        if pos.is_some() {
            self.change_brick(pos.unwrap(), BrickType::Food);
        }
    }

    fn draw_points(&self) {
        TL::jump(2, self.y_size);
        BrickVisuals{character: ' ', fg_colour: Colour::Magenta, bg_colour: Colour::White}.apply();
        print!(" {} ", self.snake.len() - INITIAL_SIZE);        
    }

    pub fn draw_ending_message(&self) {
        TL::jump(self.x_size + 1, 0);
        BrickVisuals{character: ' ', fg_colour: Colour::Cyan, bg_colour: Colour::White}.apply();
        println!(" game over, your score: {} ", self.snake.len() - INITIAL_SIZE);
        TL::jump(self.x_size + 2, 0);
        println!(" press any key to continue ");
    }

    pub fn step(&mut self) -> bool {
        self.last_step = self.facing;
        let new_pos = self.get_head().move_dir(self.last_step);

        match self.board[new_pos.0][new_pos.1] {
            BrickType::None => {
                self.step_snake(new_pos, false);
                self.end_drawing_session();
                true
            },
            BrickType::Wall => false,
            BrickType::Snake => false,
            BrickType::SnakeHead => false,
            BrickType::Food => {
                self.step_snake(new_pos, true);
                self.draw_points();
                self.spawn_food();
                self.end_drawing_session();
                true
            }
        }
    }

}

