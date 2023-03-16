
use crate::event::*;
use crate::voiceline::*;

use std::time::Duration;
use std::sync::{Arc, atomic::{AtomicU32, Ordering}, mpsc};
use std::thread;

#[derive(PartialEq)]
pub enum State {
    RUNNING,
    WAITING
}
pub struct Scheduler {
    pub current_priority : Arc<AtomicU32>,
    state : State,
    sounds : Arc<SoundStorage>,
    worker : Option<thread::JoinHandle<()>>,
}

impl Scheduler {
    pub fn new() -> Self {
        let new = Self {
            current_priority : Arc::new(AtomicU32::new(u32::max_value())),
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
        if e.priority < self.current_priority.load(Ordering::Acquire) || self.state == State::WAITING {
            self.current_priority.store(e.priority, Ordering::Release);
            let ss = self.sounds.clone();
            let cp = self.current_priority.clone();
            self.worker = Some(thread::spawn(move || {
                let res = generate_voiceline(&ss, &e);
                thread::sleep(Duration::new(0, 100000));
                if cp.load(Ordering::Acquire).clone() < e.priority {
                    return;
                }
                println!("{res}");
            }));
        }
    }
}
