use std::cell::Ref;

use crate::window::*;
use crate::board::*;
use crate::visuals::*;

pub struct SnakeWindow<'a> {
    win: Window,
    board: Board,
    visuals: Ref<'a, SnakeVisuals>,
}

impl SnakeWindow<'_> {

    pub fn new_default(context: &NcursesContext, x_size: usize, y_size: usize) -> Option<SnakeWindow> {
        Self::new(context, Board::new_default(x_size, y_size))
    }

    pub fn new(context: &NcursesContext, board: Board) -> Option<SnakeWindow> {
        Some(SnakeWindow {
            win: Window::new(0, 0, board.x_size() as i32 + 2, board.y_size() as i32 + 8),
            board: board,
            visuals: context.get_visuals().snake_visuals.borrow(),
        })
    }

    fn end_drawing_session(&self) {
        self.win.refresh();
    }

    fn change_brick(&mut self, Position(x, y): Position, category: BrickType) {
        let visual = self.visuals.get(&category);
        self.board[x][y] = category;
        self.win.move_put(x as i32, y as i32, visual);
    }

    pub fn draw_board(&self) {
        for i in 0..self.board.x_size() {
            self.win.move_cur(i as i32, 0);
            for brick in &self.board[i] {
                self.win.put_character(self.visuals.get(brick));
            }
        }
        self.draw_points();
        self.end_drawing_session();
    }

    pub fn handle_keypress(&mut self, key: i32) {
        match key {
            ncurses::KEY_UP    => { self.turn(Direction::Up);    }
            ncurses::KEY_RIGHT => { self.turn(Direction::Right); }
            ncurses::KEY_DOWN  => { self.turn(Direction::Down);  }
            ncurses::KEY_LEFT  => { self.turn(Direction::Left);  }
            _ => {}
        }
    }

    fn turn(&mut self, dir: Direction) {
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
        self.board.snake_pos = new_pos;
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
        self.win.move_print(2, self.board.y_size() as i32 + 1, &points_str);
        self.win.clear_attr();
    }

    pub fn draw_ending_message(&self) {
        let game_over = format!(" game over, your score: {} ", self.board.snake.len() - self.board.initial_size);
        self.win.set_attr(self.visuals.colors_ending.into());
        self.win.move_print(self.board.x_size() as i32, 0, &game_over);
        self.win.move_print(self.board.x_size() as i32 + 1, 0, " press any key to continue ");
        self.win.clear_attr();
        self.end_drawing_session();
    }

    pub fn step(&mut self) -> bool {
        self.board.last_step = self.board.facing;
        let new_pos = self.board.get_head().move_dir(self.board.last_step);

        let brick = &self.board.board[new_pos.0][new_pos.1];
        match brick {
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
            },
            BrickType::Portal(data) => {
                self.board.snake_pos = data.destination;
                self.board.facing = Direction::from_primitive(self.board.facing as u8 + data.rotation);
                self.step()
            }
        }
    }

}

