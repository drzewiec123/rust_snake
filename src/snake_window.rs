use crate::window::*;
use std::vec;
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
enum BrickType {
    None,
    Wall,
    Snake(Direction),
    SnakeHead(Direction),
    Food
}

static INITIAL_SIZE: usize = 4;

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
    win: Window,
    x_size: usize,
    y_size: usize,
    facing: Direction,
    last_step: Direction,
    board: Vec::<Vec::<BrickType>>,
    snake: VecDeque<Position>,
    visuals: VisualsRegistry,
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
            self.board[i][y] = BrickType::Snake(Direction::Up);
            self.snake.push_front(Position(i, y));
        }
        self.board[x][y] = BrickType::SnakeHead(Direction::Up);
    }

    fn get_head(&self) -> Position {
        *self.snake.front().unwrap()
    }

    pub fn new(x_size: usize, y_size: usize) -> Option<SnakeWindow> {
        let mut sw = SnakeWindow {
            win: Window::new(0, 0, x_size as i32 + 2, y_size as i32 + 8),
            x_size: x_size,
            y_size: y_size,
            facing: Direction::Up,
            last_step: Direction::Up,
            board: Self::create_board(x_size, y_size),
            snake: VecDeque::new(),
            visuals: VisualsRegistry::build()?
        };
        sw.place_snake(x_size / 2, y_size / 2);
        let pos = sw.find_valid_food_spawn().unwrap();
        sw.board[pos.0][pos.1] = BrickType::Food;
        Some(sw)
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
        for (i, row) in self.board.iter().enumerate() {
            self.win.move_cur(i as i32, 0);
            for brick in row {
                self.win.put_character(self.visuals.get(*brick));
            }
        }
        self.draw_points();
        self.end_drawing_session();
    }

    pub fn turn(&mut self, dir: Direction) {
        if (dir as u8) != (self.last_step as u8 + 2) % 4 {
            self.facing = dir;
            self.change_brick(self.get_head(), BrickType::SnakeHead(dir));
            self.end_drawing_session();
        }
    }

    fn step_snake(&mut self, new_pos: Position, grow: bool) {
        if !grow {
            let back = self.snake.pop_back().unwrap();
            self.change_brick(back, BrickType::None);
        }

        self.snake.push_front(new_pos);
        self.change_brick(new_pos, BrickType::SnakeHead(self.facing));
        self.change_brick(self.snake[1], BrickType::Snake(self.facing));
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
        let points_str = format!(" {} ", self.snake.len() - INITIAL_SIZE);
        self.win.set_attr(self.visuals.colors_points.into());
        self.win.move_print(2, self.y_size as i32 + 1, &points_str);
        self.win.clear_attr();
    }

    pub fn draw_ending_message(&self) {
        let game_over = format!(" game over, your score: {} ", self.snake.len() - INITIAL_SIZE);
        self.win.set_attr(self.visuals.colors_ending.into());
        self.win.move_print(self.x_size as i32, 0, &game_over);
        self.win.move_print(self.x_size as i32 + 1, 0, " press any key to continue ");
        self.win.clear_attr();
        self.end_drawing_session();
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

