use complaint::Complaint;
use guard::Guard;
use std::collections::HashMap;
use std::sync::mpsc::Sender;



/// A simple type definition for consistency
pub type PrisonerVec = Vec<Prisoner>;

pub struct Prisoner {
    /// The name of the prisoner
    pub name: String,
    /// A hashmap mapping a need to the respective sender object of the guard
    guard_map: HashMap< String, Sender< Complaint > >,
}

impl Prisoner {
    /// Sends a complaint to the respective guard
    pub fn complain(&self, complaint: Complaint) {
        self.guard_map[&complaint.need].send(complaint).unwrap();
    }

    /// Adds a guard to the guard map
    pub fn add_guard(&mut self, guard: &Guard) {
        self.guard_map.insert(guard.need.clone(), guard.get_complaint_sender());
    }

    /// Constructs a new `PrisonerVec`
    ///
    /// # Examples
    ///
    /// let name_vector = vec![
    ///     "Alfred".to_string(),
    ///     "Jim".to_string(),
    ///     "Henry".to_string(),
    /// ];
    ///
    /// let prisoner_vector = Prisoner::new_vec(name_vec);
    pub fn new_vec(name_vec: Vec<String>) -> PrisonerVec {
        name_vec.into_iter().map(|name| {
            Prisoner::new(name.as_str())
        }).collect()
    }

    /// Constructs a new `Prisoner`
    ///
    /// # Examples
    ///
    /// let prisoner = Prisoner::new("Jacob");
    pub fn new(name: &str) -> Prisoner {
        Prisoner {
            name: name.to_string(),
            guard_map: HashMap::new()
        }
    }
}
