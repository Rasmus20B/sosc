use priority_queue::PriorityQueue;

#[derive(Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Event {
    pub name : String,
    pub id : u32,
    pub priority : u8
}
struct EventQueue { 
    q : PriorityQueue<Event, u8>
}

impl EventQueue {
    pub fn pop_event() {
        
    }

    pub fn store_event(e : Event) {
    }
}

