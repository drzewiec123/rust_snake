use std::io::Read;
use crate::snake_window::{Direction, Position, Board, BrickType};

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
    pub fn from_line(line: String) -> Option<Bar> {
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

    fn new(x_size: usize, y_size: usize) -> Option<BoardBuilder> {
        Some(BoardBuilder { board: Board::new_empty(x_size, y_size) })
    }
}
