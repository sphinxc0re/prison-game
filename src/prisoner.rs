use std::sync::mpsc::Sender;
use std::collections::HashMap;
use guard::Guard;
use complaint::Complaint;



pub type PrisonerVec = Vec<Prisoner>;

pub struct Prisoner {
    pub name: String,
    guard_map: HashMap< String, Sender< Complaint > >,
}

impl Prisoner {
    pub fn complain(&self, complaint: Complaint) {
        self.guard_map[&complaint.need].send(complaint).unwrap();
    }

    pub fn add_guard(&mut self, guard: &Guard) {
        self.guard_map.insert(guard.need.clone(), guard.get_complaint_sender());
    }

    pub fn new(name: &str) -> Prisoner {
        Prisoner {
            name: name.to_string(),
            guard_map: HashMap::new()
        }
    }
}
