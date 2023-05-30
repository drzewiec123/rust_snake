mod terminal_tools;
mod snake_window;

use crate::snake_window::SnakeWindow;

fn main() {
    let mut game_window = SnakeWindow::new(15, 30);
    game_window.draw_board();
    game_window.step();
    //game_window.step();
    terminal_tools::set_colour(terminal_tools::Colour::RESET);
    terminal_tools::set_bg_colour(terminal_tools::Colour::RESET);
    terminal_tools::jump(15, 0);
    println!()
}
