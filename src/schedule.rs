
use crate::event::*;
use crate::voiceline::*;

use std::time::Duration;
use std::sync::{Arc, atomic, mpsc};
use std::thread;

#[derive(PartialEq)]
pub enum State {
    RUNNING,
    WAITING
}
pub struct Scheduler {
    pub current_priority : atomic::AtomicU32,
    state : State,
    sounds : Arc<SoundStorage>,
    worker : Option<thread::JoinHandle<()>>,
}

impl Scheduler {
    pub fn new() -> Self {
        let new = Self {
            current_priority : atomic::AtomicU32::new(0),
            state: State::WAITING,
            sounds : Arc::new(SoundStorage { store : Vec::new() }),
            worker : None,
        };
        new
    }


    pub fn start(&mut self) {
        self.worker = Some(thread::spawn( move || {
        }
        ));

    }

    pub fn store_sounds(&mut self, p : &std::path::Path) {
        self.sounds = Arc::new(store_segments(&p).unwrap())
    }
    pub fn add_event(&mut self, e : Event) {
        // Do the checks here for the current event in the scheduler
        // If it passes, interrupe the schuler and send the new one instead
        if e.priority < self.current_priority.load(atomic::Ordering::Relaxed) || self.state == State::WAITING {
            self.current_priority.store(e.priority, atomic::Ordering::Release);
            let res = generate_voiceline(&self.sounds, &e);
            println!("{res}");
            drop(&self.worker.take());
        }
    }
}
