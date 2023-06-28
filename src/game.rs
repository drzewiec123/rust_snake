use std::{time, thread};

use crate::board_file;
use crate::event_emitter::EventEmitter;
use crate::selection_window::{SelectionWindow, SelectionWindowEvent, SelectionWindowEventId};
use crate::basic_window::{Window, NcursesContext, Dimensions};
use crate::snake_window::SnakeWindow;

pub struct Game<'a> {
    context: &'a NcursesContext,
}

impl Game<'_> {

    pub fn new<'b>(context: &'b NcursesContext) -> Game<'b> {
        Game {
            context,
        }
    }

    fn run_game(&self, mut win: SnakeWindow) {
        win.draw();
        self.context.clear_key_queue();
        self.context.get_key();
        let wait_time = time::Duration::from_millis(300);
        loop {
            thread::sleep(wait_time);
            let mut key: Option<i32>;
            while { key = self.context.get_last_pressed_key(); key.is_some() } {
                win.handle_keypress(key.unwrap());
                win.refresh();
            }
            if win.step() {
                win.refresh();
                break;
            }
            win.refresh();
        }
        self.context.clear_key_queue();
        self.context.get_key();
    }

    fn default_board(&self) -> SnakeWindow {
        SnakeWindow::new_default(self.context, 15, 30)
    }

    fn from_file(&self, file: &str) -> Option<SnakeWindow> {
        let path = "boards/".to_owned() + file + ".board";
        Some(SnakeWindow::new(self.context, board_file::from_file(path.as_str())?))
    }

    fn run_menu(&self, mut win: SelectionWindow) -> Option<SnakeWindow> {
        win.draw();
        let mut exit = false;
        let mut snake_win: Option<SnakeWindow> = None;

        while !exit {

            let mut event_callback = |event: SelectionWindowEvent| {
                match event {
                    SelectionWindowEvent::Select(opt) => { 
                        match opt.as_str() {
                            "Exit" => { exit = true; },
                            "Default" => { snake_win = Some(self.default_board()); exit = true; },
                            _ => { snake_win = self.from_file(opt.as_str()); exit = true; }
                        }
                    },
                }
            };

            win.handle_keypress(self.context.get_key());
            win.refresh();
            win.get_pool().handle_events(&mut event_callback);
        }
        snake_win
    }

    pub fn run(&mut self) {
        loop {
            let options: Vec<String> = ["Default", "u_pattern", "simple_portal", "Exit"].into_iter().map(String::from).collect();
            let mut menu = SelectionWindow::new_selected(
                self.context, Dimensions::new(0, 0, 6, 17), options, Some(0)
            );
            menu.get_pool().listen(&[SelectionWindowEventId::SelectId]);
            let board = self.run_menu(menu);
            if board.is_none() {
                break;
            }

            self.run_game(board.unwrap());
        }
    }

}
