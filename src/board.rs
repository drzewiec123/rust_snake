use std::vec;
use std::ops::{Index, IndexMut};
use std::collections::VecDeque;
use rand::seq::SliceRandom;

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Direction {
    Up = 0,
    Right,
    Down,
    Left
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum BrickType {
    None,
    Wall,
    Snake(Direction),
    SnakeHead(Direction),
    Food
}

#[derive(Copy, Clone)]
pub struct Position(pub usize, pub usize);

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

pub struct Board {
    x_size: usize,
    y_size: usize,
    pub facing: Direction,
    pub last_step: Direction,
    board: Vec::<Vec::<BrickType>>,
    pub snake: VecDeque<Position>,
    pub initial_size: usize,
}

impl Index<usize> for Board {
    type Output = Vec::<BrickType>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.board[index]
    }
}

impl Index<Position> for Board {
    type Output = BrickType;

    fn index(&self, Position(x, y): Position) -> &Self::Output {
        &self[x][y]
    }
}

impl IndexMut<usize> for Board {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.board[index]
    }
}

impl IndexMut<Position> for Board {
    fn index_mut(&mut self, Position(x, y): Position) -> &mut Self::Output {
        &mut self[x][y]
    }
}

impl Board {
    pub fn new_empty(x_size: usize, y_size: usize) -> Board {
        Board {
            x_size,
            y_size,
            facing: Direction::Up,
            last_step: Direction::Up,
            board: vec![vec![BrickType::None; y_size]; x_size],
            snake: VecDeque::new(),
            initial_size: 0
        }
    }

    pub fn new_default(x_size: usize, y_size: usize) -> Board {
        let mut b = Self::new_empty(x_size, y_size);
        b.create_wall_outline();
        b.initial_size = 4;
        b.place_snake(x_size / 2, y_size / 2);
        let pos = b.find_valid_food_spawn().unwrap();
        b[pos] = BrickType::Food;
        b
    }

    fn place_snake(&mut self, x: usize, y: usize) {
        for i in (x..x + self.initial_size).rev() {
            self[i][y] = BrickType::Snake(Direction::Up);
            self.snake.push_front(Position(i, y));
        }
        self.board[x][y] = BrickType::SnakeHead(Direction::Up);
    }

    fn create_wall_outline(&mut self) {
        self[0].fill(BrickType::Wall);
        self.board[self.x_size - 1].fill(BrickType::Wall);
        for i in 1..self.x_size - 1 {
            self.board[i][0] = BrickType::Wall;
            self.board[i][self.y_size - 1] = BrickType::Wall;
        };
    }

    pub fn find_valid_food_spawn(&mut self) -> Option<Position> {
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

    pub fn get_head(&self) -> Position {
        *self.snake.front().unwrap()
    }

    pub fn x_size(&self) -> usize {
        self.x_size
    }

    pub fn y_size(&self) -> usize {
        self.y_size
    }
}