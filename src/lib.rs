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
    // pub fn voicelines() {
    //     let path = Path::new("assets/vls.yaml");
    //     let s : Vec<Voiceline> = vec!();
    //     let mut storage = SoundStorage {
    //         store : s
    //     };
    //     let vls_result = &voiceline::store_segments(path, &mut storage).unwrap();
    //     assert_eq!(vls_result.store[0].id, 0);
    //
    //     let ans = generate_voiceline(&vls_result, &event::Event { name: "goal".to_string(), id: 1, priority: 1});
    //
    //     println!("{}", ans);
    // }
    
    pub fn sosc_main_test() {
        let mut h = sosc::init();
        let e : Event = Event{ name: "match_start".to_string(), id : 1, priority : 2 }; 
        h.store_sounds(Path::new("assets/vls.yaml"));
        h.start();
        h.add_event(e);
        let e : Event = Event{ name: "Goal".to_string(), id : 2, priority : 3 }; 
        h.add_event(e);
        sleep(Duration::new(0, 100000000));
    }
}
