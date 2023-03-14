
use crate::event::*;
use crate::voiceline::*;

use std::time::{Instant, Duration};
use std::thread;

pub struct Scheduler {
    pub current_priority : u32,
    pub current_lifetime : Duration,
    sounds : SoundStorage
}

impl Scheduler {
    pub fn new() -> Self {
        let new = Self {
            current_lifetime : Duration::new(0, 0),
            current_priority : 0,
            sounds : SoundStorage { store : Vec::new() } 
        };
        new
    }
    pub fn store_sounds(&mut self, p : &std::path::Path) {
        self.sounds = store_segments(&p).unwrap();
    }
    pub fn add_event(&mut self, e : &Event) {
        // Do the checks here for the current event in the scheduler
        // If it passes, interrupe the schuler and send the new one instead
        if e.priority >= self.current_priority {
            self.current_priority = e.priority;
            let res = generate_voiceline(&self.sounds, e);
            println!("{res}");
        }
    }
}
