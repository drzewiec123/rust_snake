use crate::board::*;
use std::{fs::File, io::BufRead};
use std::io::BufReader;

#[derive(Copy, Clone)]
struct Bar {
    type_id: char,
    pos: Position,
    len: usize,
    dir: Direction,
}

fn from_primitive(x: u8) -> Option<Direction> {
    match x {
        x if x == Direction::Up as u8 => Some(Direction::Up),
        x if x == Direction::Right as u8 => Some(Direction::Right),
        x if x == Direction::Down as u8 => Some(Direction::Down),
        x if x == Direction::Left as u8 => Some(Direction::Left),
        _ => None
    }
}

impl Bar {
    fn from_line(line: String) -> Option<Bar> {
        let mut iter = line.split_whitespace().into_iter();
        let bg = iter.next()?;
        if bg.len() != 1 {
            return None
        }
        let type_id: char = bg.chars().next()?;
        let x: usize = iter.next()?.parse().ok()?;
        let y: usize = iter.next()?.parse().ok()?;
        let len: usize = iter.next()?.parse().ok()?;
        let dir: Direction = from_primitive(iter.next()?.parse().ok()?)?;
        Some(Bar { type_id, pos: Position(x, y), len, dir })
    }
}

struct BoardBuilder {
    board: Board
}

impl BoardBuilder {
    fn build(mut self) -> Board {
        if !self.board.snake.is_empty() {
            let head = self.board.get_head();
            self.board[head] = BrickType::SnakeHead(self.board.facing);
        }
        let pos = self.board.find_valid_food_spawn().unwrap();
        self.board[pos] = BrickType::Food;
        self.board.initial_size = self.board.snake.len();
        self.board
    }

    fn add_bar(&mut self, bar: Bar) {
        match bar.type_id {
            'W' => {
                let mut pos = bar.pos;
                for _ in 0..bar.len {
                    self.board[pos] = BrickType::Wall;
                    pos = pos.move_dir(bar.dir);
                }
            },
            'S' => {
                let mut pos = bar.pos;
                for _ in 0..bar.len {
                    self.board[pos] = BrickType::Snake(bar.dir);
                    self.board.snake.push_front(pos);
                    pos = pos.move_dir(bar.dir);
                }
                self.board.facing = bar.dir;
                self.board.last_step = bar.dir;
            },
            _ => {}
        };
    }

    fn new(x_size: usize, y_size: usize) -> BoardBuilder {
        BoardBuilder { board: Board::new_empty(x_size, y_size) }
    }
}

pub fn from_file(file_path: &str) -> Option<Board> {
    let file = File::open(file_path).ok()?;
    let reader = BufReader::new(file);
    let mut lines_iter = reader.lines();

    let line = lines_iter.next()?.ok()?;
    let mut iter = line.split_whitespace();
    let x_size: usize = iter.next()?.parse().ok()?;
    let y_size: usize = iter.next()?.parse().ok()?;

    let mut builder = BoardBuilder::new(x_size, y_size);
    for line in lines_iter {
        let line = line.ok()?;
        if line.is_empty() {
            continue;
        }
        let bar = Bar::from_line(line)?;
        builder.add_bar(bar);
    }
    Some(builder.build())
}
