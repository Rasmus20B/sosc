pub mod voiceline;
pub mod event;
pub mod sosc;
pub mod schedule;
use crate::voiceline::*;
use crate::event::*;
use crate::sosc::*;
use std::path::*;

use std::thread::*;
use std::time::Duration;

#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    pub fn sosc_main_test() {
        let mut h = sosc::init();
        let e : Event = Event{ name: "match_start".to_string(), id : 1, priority : 1 }; 
        h.store_sounds(Path::new("assets/vls.yaml"));
        h.add_event(e);
        sleep(Duration::new(0, 200));
        // preempt previous event voiceline generation (higher priority)
        let e : Event = Event{ name: "Goal".to_string(), id : 2, priority : 1 }; 
        h.add_event(e);
        sleep(Duration::new(0, 1000000));
    }
}
