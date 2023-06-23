use crate::board::*;
use std::{fs::File, io::BufRead};
use std::io::BufReader;

#[derive(Copy, Clone)]
struct SimpleBar {
    pos: Position,
    len: usize,
    dir: Direction,
}

impl SimpleBar {
    pub fn from_line<'a, I>(iter: &mut I) -> Option<SimpleBar> 
    where
        I: Iterator<Item = &'a str>
    {
        let x: usize = iter.next()?.parse().ok()?;
        let y: usize = iter.next()?.parse().ok()?;
        let len: usize = iter.next()?.parse().ok()?;
        let dir: Direction = Direction::from_primitive(iter.next()?.parse().ok()?);
        Some(SimpleBar { pos: Position(x, y), len, dir })
    }
}

struct WallBar {
    base: SimpleBar
}

impl WallBar {
    pub fn from_line<'a, I>(iter: &mut I) -> Option<WallBar> 
    where
        I: Iterator<Item = &'a str>
    {
        Some(WallBar { base: SimpleBar::from_line(iter)? })
    }

    pub fn apply(&self, board: &mut Board) {
        let mut pos = self.base.pos;
        for _ in 0..self.base.len {
            board[pos] = BrickType::Wall;
            pos = pos.move_dir(self.base.dir);
        }
    }
}

struct SnakeBar {
    base: SimpleBar
}

impl SnakeBar {
    pub fn from_line<'a, I>(iter: &mut I) -> Option<SnakeBar> 
    where
        I: Iterator<Item = &'a str>
    {
        Some(SnakeBar { base: SimpleBar::from_line(iter)? })
    }

    pub fn apply(&self, board: &mut Board) {
        let mut pos = self.base.pos;
        for _ in 0..self.base.len {
            board[pos] = BrickType::Snake(self.base.dir);
            board.snake.push_front(pos);
            pos = pos.move_dir(self.base.dir);
        }
        board.facing = self.base.dir;
        board.last_step = self.base.dir;
    }
}

struct PortalBar {
    base: SimpleBar,
    destination: Position,
    rotation: u8,
    mirror: bool,
    colour: i16,
}

impl PortalBar {
    pub fn from_line<'a, I>(iter: &mut I) -> Option<PortalBar> 
    where
        I: Iterator<Item = &'a str>
    {
        let basic_bar = SimpleBar::from_line(iter)?;
        let x: usize = iter.next()?.parse().ok()?;
        let y: usize = iter.next()?.parse().ok()?;
        let rotation: u8 = iter.next()?.parse().ok()?;
        let mirror: u8 = iter.next()?.parse().ok()?;
        let colour: i16 = iter.next()?.parse().ok()?;

        Some(PortalBar {
            base: basic_bar,
            destination: Position(x, y),
            mirror: mirror != 0,
            rotation: rotation % 4,
            colour,
        })
    }

    pub fn apply(&self, board: &mut Board) {
        let mut source_pos = self.base.pos;
        let mut dest_pos = self.destination;
        let source_dir = self.base.dir;
        let dest_dir = self.base.dir.rotate(self.rotation);
        let out_rotation = if self.mirror { self.rotation + 2 } else { self.rotation };
        for _ in 0..self.base.len {
            board[source_pos] = BrickType::Portal(
                Box::new(PortalData {
                    destination: dest_pos,
                    colour: self.colour,
                    rotation: out_rotation
                })
            );
            source_pos = source_pos.move_dir(source_dir);
            dest_pos = dest_pos.move_dir(dest_dir);
        }
    }
}

struct BoardBuilder {
    board: Board
}

impl BoardBuilder {
    fn build(mut self) -> Board {
        if !self.board.snake.is_empty() {
            let head = *self.board.snake.front().unwrap();
            self.board.snake_pos = head;
            self.board[head] = BrickType::SnakeHead(self.board.facing);
        }
        let pos = self.board.find_valid_food_spawn().unwrap();
        self.board[pos] = BrickType::Food;
        self.board.initial_size = self.board.snake.len();
        self.board
    }

    fn add_bar(&mut self, line: String) -> Option<()> {
        let mut iter = line.split_whitespace();
        let bg = iter.next()?;
        if bg.len() != 1 {
            return None
        }
        let type_id: char = bg.chars().next()?;
        match type_id {
            'W' => {
                WallBar::from_line(&mut iter)?.apply(&mut self.board);
            },
            'S' => {
                SnakeBar::from_line(&mut iter)?.apply(&mut self.board);
            },
            'P' => {
                PortalBar::from_line(&mut iter)?.apply(&mut self.board);
            },
            _ => {}
        };
        Some(())
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
        builder.add_bar(line);
    }
    Some(builder.build())
}
