// main interface
//
use crate::event::*;
use crate::schedule::Scheduler;
use crate::voiceline::*;

use std::time::Duration;

pub fn init() -> Scheduler {
   return Scheduler::new(); 
}
fn stop() {

}
