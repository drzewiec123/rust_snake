use std::rc::Rc;

use crate::basic_window::*;
use crate::event_emitter::{Event, EventPool, EventEmitter};
use crate::visuals::*;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum SelectionWindowEventId {
    SelectId,
}

pub enum SelectionWindowEvent {
    Select(String)
}

impl Event for SelectionWindowEvent {
    type EventId = SelectionWindowEventId;

    fn id(&self) -> Self::EventId {
        match *self {
            SelectionWindowEvent::Select(_) => SelectionWindowEventId::SelectId,
        }
    }
}

pub struct SelectionWindow {
    win: BasicWindow,
    event_pool: EventPool<SelectionWindowEvent>,
    options: Vec<String>,
    selection: Option<usize>,
    common_visuals: Rc<CommonVisuals>,
}

impl EventEmitter<SelectionWindowEvent> for SelectionWindow {
    fn get_pool(&mut self) -> &mut EventPool<SelectionWindowEvent> {
        &mut self.event_pool
    }
}

impl SelectionWindow {
    pub fn new(context: &NcursesContext, dim: Dimensions, opts: Vec<String>) -> SelectionWindow {
        Self::new_selected(context, dim, opts, None)
    }

    pub fn new_selected(context: &NcursesContext, dim: Dimensions, opts: Vec<String>, sel: Option<usize>) -> SelectionWindow {
        SelectionWindow {
            win: BasicWindow::new(dim),
            event_pool: EventPool::new(),
            options: opts,
            selection: sel,
            common_visuals: Rc::clone(&context.get_visuals().common_visuals),
        }
    }

    fn draw_option(&self, i: usize) {
        let mut attr: Attributes = self.common_visuals.get_colour(ncurses::COLOR_WHITE).into();
        if self.selection.is_some() && self.selection.unwrap() == i {
            attr = attr | ncurses::A_REVERSE();
        }
        self.win.apply_attr(attr);
        self.win.move_print((i + 1) as i32, 2, &self.options[i]);
        self.win.clear_attr();
    }

    pub fn get_selected(&self) -> Option<&String> {
        Some(&self.options[self.selection?])
    }

}

impl Window for SelectionWindow {
    fn refresh(&self) {
        self.win.refresh();
    }

    fn handle_keypress(&mut self, key: i32) {
        if self.selection.is_none() {
            if !self.options.is_empty() {
                self.selection = Some(0);
                self.draw_option(0)
            }
        } else {
            let sel = self.selection.unwrap();
            match key {
                ncurses::KEY_UP => { self.selection = if sel == 0 {Some(self.options.len() - 1)} else {Some(sel - 1)} },
                ncurses::KEY_DOWN => { self.selection = Some((sel + 1) % self.options.len()) },
                10 /*NEWLINE*/ => { self.event_pool.push_event(SelectionWindowEvent::Select(self.options[sel].clone())); }
                _ => { println!("{}", key); ncurses::getch(); panic!(); }
            }
            if sel != self.selection.unwrap() {
                self.draw_option(sel);
                self.draw_option(self.selection.unwrap());
            }
        }
    }

    fn draw(&self) {
        for i in 0..self.options.len() {
            self.draw_option(i);
        }
        self.refresh();
    }

}
