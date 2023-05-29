use crate::terminal_tools as TL;

#[derive(Copy, Clone)]
struct Position(i32, i32);

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Direction {
    UP,
    RIGHT,
    DOWN,
    LEFT
}
pub struct SnakeWindow {
    x_size: i32,
    y_size: i32,
    head_pos: Position,
    facing: Direction,
    last_step: Direction,

}

impl SnakeWindow {

    pub fn new(x_size: i32, y_size: i32) -> Self {
        let sw = SnakeWindow {
            x_size: x_size,
            y_size: y_size,
            head_pos: Position(x_size / 2, y_size / 2),
            facing: Direction::UP,
            last_step: Direction::UP
        };
        sw.draw_frame();
        sw.draw_snake();
        TL::flush();
        return sw;
    }

    pub fn draw_frame(&self) {

    }

    pub fn draw_snake(&self) {
        
    }

    fn update_snake(&self) {
        TL::flush()
    }

    pub fn turn(&mut self, dir: Direction) {
        if dir != self.last_step {
            self.facing = dir;
        }
    }

    pub fn step(&mut self) {
        self.last_step = self.facing;
        self.update_snake()
    }



}

