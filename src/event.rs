use crate::schedule;
use std::time::{Duration};


#[derive(Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Event {
    pub name : String,
    pub id : u32,
    pub priority : u32,
}

// Add event function needs to call to scheduler
pub fn add_event() {
    // if the conditions are met, then interrupt the worker thread
}
