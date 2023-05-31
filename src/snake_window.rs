use crate::terminal_tools as TL;
use std::vec;
use std::collections::VecDeque;
use rand::seq::SliceRandom;

#[derive(Copy, Clone, PartialEq, Eq)]
enum BrickType {
    NONE,
    WALL,
    SNAKE,
    SNAKE_HEAD,
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
        BrickType::NONE       => BrickVisuals { character: ' ', fg_colour: TL::Colour::BLACK,  bg_colour: TL::Colour::BLACK },
        BrickType::WALL       => BrickVisuals { character: '█', fg_colour: TL::Colour::BLUE,   bg_colour: TL::Colour::BLACK },
        BrickType::SNAKE      => BrickVisuals { character: '%', fg_colour: TL::Colour::RED,    bg_colour: TL::Colour::BLACK },
        BrickType::SNAKE_HEAD => BrickVisuals { character: '%', fg_colour: TL::Colour::YELLOW, bg_colour: TL::Colour::BLACK },
        BrickType::FOOD       => BrickVisuals { character: '*', fg_colour: TL::Colour::GREEN,  bg_colour: TL::Colour::BLACK },
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
    facing: Direction,
    last_step: Direction,
    board: Vec::<Vec::<BrickType>>,
    snake: VecDeque<Position>
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

    fn place_snake(&mut self, x: usize, y: usize) {
        for i in (x..x + 4).rev() {
            self.board[i][y] = BrickType::SNAKE;
            self.snake.push_front(Position(i, y));
        }
        self.board[x][y] = BrickType::SNAKE_HEAD;
    }

    fn get_head(&self) -> Position {
        *self.snake.front().unwrap()
    }

    pub fn new(x_size: usize, y_size: usize) -> Self {
        let mut sw = SnakeWindow {
            x_size: x_size,
            y_size: y_size,
            facing: Direction::UP,
            last_step: Direction::UP,
            board: Self::create_board(x_size, y_size),
            snake: VecDeque::new()
        };
        sw.place_snake(x_size / 2, y_size / 2);
        let pos = sw.find_valid_food_spawn().unwrap();
        sw.board[pos.0][pos.1] = BrickType::FOOD;
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

    fn step_snake(&mut self, new_pos: Position, grow: bool) {
        if !grow {
            let back = self.snake.pop_back().unwrap();
            self.change_brick(back, BrickType::NONE);
        }

        self.snake.push_front(new_pos);
        self.change_brick(new_pos, BrickType::SNAKE_HEAD);
        self.change_brick(self.snake[1], BrickType::SNAKE);
    }

    fn find_valid_food_spawn(&mut self) -> Option<Position> {
        let mut empty_bricks = Vec::<Position>::new();
        for x in 0..self.board.len() {
            for y in 0..self.board[x].len() {
                if self.board[x][y] == BrickType::NONE {
                    empty_bricks.push(Position(x, y));
                }
            }
        }
        empty_bricks.choose(&mut rand::thread_rng()).cloned()
    }

    fn spawn_food(&mut self) {
        let pos = self.find_valid_food_spawn();
        if pos.is_some() {
            self.change_brick(pos.unwrap(), BrickType::FOOD);
        }
    }

    pub fn step(&mut self) -> bool {
        self.last_step = self.facing;
        let new_pos = self.get_head().move_dir(self.last_step);

        match self.board[new_pos.0][new_pos.1] {
            BrickType::NONE => {
                self.step_snake(new_pos, false);
                self.end_drawing_session();
                true
            },
            BrickType::WALL => false,
            BrickType::SNAKE => false,
            BrickType::SNAKE_HEAD => false,
            BrickType::FOOD => {
                self.step_snake(new_pos, true);
                self.spawn_food();
                self.end_drawing_session();
                true
            }
        }
    }



}

