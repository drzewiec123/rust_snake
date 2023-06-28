use core::panic;
use std::hash::Hash;
use std::collections::{VecDeque, HashSet};


pub trait Event {
    type EventId: Hash + Eq + Clone;

    fn id(&self) -> Self::EventId;
}

pub struct EventPool<EventType: Event> {
    queue: VecDeque<EventType>,
    listened_events: HashSet<EventType::EventId>
}

impl<EventType: Event> EventPool<EventType> {
    pub fn new() -> Self {
        Self::new_listened(&[])
    }

    pub fn new_listened(listened_events: &[EventType::EventId]) -> Self {
        EventPool {
            queue: VecDeque::new(),
            listened_events: HashSet::from_iter(listened_events.iter().cloned()),
        }
    }

    pub fn listen(&mut self, ids: &[EventType::EventId]) {
        for id in ids.iter() {
            self.listened_events.insert(id.clone());
        }
    }

    pub fn clear(&mut self) {
        self.queue.clear();
    }

    pub fn push_event(&mut self, event: EventType) {
        if self.listened_events.contains(&event.id()) {
            self.queue.push_back(event);
        }
    }

    pub fn handle_events(&mut self, handler: &mut dyn FnMut(EventType) -> ()) {
        loop {
            let event = self.queue.pop_front();
            if event.is_some() {
                handler(event.unwrap());
            } else {
                break;
            }
        }
    }
}

pub trait EventEmitter<T: Event> {
    fn get_pool(&mut self) -> &mut EventPool<T>;
}
