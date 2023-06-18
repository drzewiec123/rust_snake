use crate::window::*;
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

struct VisualsRegistry {
    none: PrintableCharacter,
    wall: PrintableCharacter,
    food: PrintableCharacter,
    colors_head: ColorPair,
    colors_body: ColorPair,
    colors_points: ColorPair,
    colors_ending: ColorPair,
}

impl VisualsRegistry {
    fn build() -> Option<VisualsRegistry> {
        Some(VisualsRegistry {
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

    fn get(&self, category: BrickType) -> PrintableCharacter {
        match category {
            BrickType::None => self.none,
            BrickType::Wall => self.wall,
            BrickType::Snake(dir) => PrintableCharacter::new(Self::get_char_from_direction(dir), self.colors_body.into()),
            BrickType::SnakeHead(dir) => PrintableCharacter::new(Self::get_char_from_direction(dir), self.colors_head.into()),
            BrickType::Food => self.food,
        }
    }
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

}

pub struct SnakeWindow {
    win: Window,
    board: Board,
    visuals: VisualsRegistry,
}

impl SnakeWindow {

    pub fn new_default(x_size: usize, y_size: usize) -> Option<SnakeWindow> {
        Self::new(Board::new_default(x_size, y_size))
    }

    pub fn new(board: Board) -> Option<SnakeWindow> {
        Some(SnakeWindow {
            win: Window::new(0, 0, board.x_size as i32 + 2, board.y_size as i32 + 8),
            board: board,
            visuals: VisualsRegistry::build()?
        })
    }

    fn end_drawing_session(&self) {
        self.win.refresh();
    }

    fn change_brick(&mut self, Position(x, y): Position, category: BrickType) {
        self.board[x][y] = category;
        let visual = self.visuals.get(category);
        self.win.move_put(x as i32, y as i32, visual);
    }

    pub fn draw_board(&self) {
        for i in 0..self.board.x_size {
            self.win.move_cur(i as i32, 0);
            for brick in &self.board[i] {
                self.win.put_character(self.visuals.get(*brick));
            }
        }
        self.draw_points();
        self.end_drawing_session();
    }

    pub fn turn(&mut self, dir: Direction) {
        if (dir as u8) != (self.board.last_step as u8 + 2) % 4 {
            self.board.facing = dir;
            self.change_brick(self.board.get_head(), BrickType::SnakeHead(dir));
            self.end_drawing_session();
        }
    }

    fn step_snake(&mut self, new_pos: Position, grow: bool) {
        if !grow {
            let back = self.board.snake.pop_back().unwrap();
            self.change_brick(back, BrickType::None);
        }

        self.board.snake.push_front(new_pos);
        self.change_brick(new_pos, BrickType::SnakeHead(self.board.facing));
        self.change_brick(self.board.snake[1], BrickType::Snake(self.board.facing));
    }

    fn spawn_food(&mut self) {
        let pos = self.board.find_valid_food_spawn();
        if pos.is_some() {
            self.change_brick(pos.unwrap(), BrickType::Food);
        }
    }

    fn draw_points(&self) {
        let points_str = format!(" {} ", self.board.snake.len() - self.board.initial_size);
        self.win.set_attr(self.visuals.colors_points.into());
        self.win.move_print(2, self.board.y_size as i32 + 1, &points_str);
        self.win.clear_attr();
    }

    pub fn draw_ending_message(&self) {
        let game_over = format!(" game over, your score: {} ", self.board.snake.len() - self.board.initial_size);
        self.win.set_attr(self.visuals.colors_ending.into());
        self.win.move_print(self.board.x_size as i32, 0, &game_over);
        self.win.move_print(self.board.x_size as i32 + 1, 0, " press any key to continue ");
        self.win.clear_attr();
        self.end_drawing_session();
    }

    pub fn step(&mut self) -> bool {
        self.board.last_step = self.board.facing;
        let new_pos = self.board.get_head().move_dir(self.board.last_step);

        match self.board[new_pos] {
            BrickType::None => {
                self.step_snake(new_pos, false);
                self.end_drawing_session();
                true
            },
            BrickType::Wall => false,
            BrickType::Snake(_) => false,
            BrickType::SnakeHead(_) => false,
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

