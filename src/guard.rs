use mp::Envelope;
use std::sync::mpsc::channel;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;



/// A simple type definition for consistency
pub type GuardVec = Vec<Guard>;

pub struct Guard {
    /// The sender channel of the guard, typed
    sender: Sender<Envelope>,
    /// The receiver channel of the guard, typed
    receiver: Receiver<Envelope>,
    /// The need, the guard is able to satisfy
    pub need: String,
    /// A map to keep track of the needs of the prisoners
    prisoners: Vec<String>
}

impl Guard {
    /// Constructs a new `Guard`
    ///
    /// # Examples
    /// ```
    /// let guard = Guard::new("health");
    /// ```
    pub fn new(need: &str) -> Guard {
        let (snd, rec) = channel();
        Guard {
            sender: snd,
            receiver: rec,
            need: need.to_string(),
            prisoners: Vec::new()
        }
    }

    /// Constructs a new `GuardVec`
    ///
    /// # Examples
    /// ```
    /// let need_vec = vec![
    ///     "health".to_string(),
    ///     "food".to_string(),
    ///     "freedom".to_string(),
    /// ];
    ///
    /// let guard_vec = Guard::new_vec(need_vec);
    /// ```
    pub fn new_vec(need_vec: Vec<String>) -> GuardVec {
        need_vec.into_iter().map(|need| {
            Self::new(need.as_str())
        }).collect()
    }

    /// Returns a copy of the complaint sender
    pub fn get_sender(&self) -> Sender<Envelope> {
        self.sender.clone()
    }

    /// Waits for a new complaint to be sent and returns it
    pub fn receive_message(&self) -> Option<Envelope> {
        self.receiver.recv().ok()
    }

    pub fn track_prisoner(&mut self, prisoner_name: &String) {
        if !self.prisoners.contains(prisoner_name) {
            self.prisoners.push((*prisoner_name).clone());
        }
    }


    pub fn untrack_prisoner(&mut self, prisoner_name: &String) {
        if self.prisoners.contains(prisoner_name) {
            let index = self.prisoners.iter().position(|x| *x == *prisoner_name).unwrap();
            self.prisoners.remove(index);
        }
    }


    pub fn tracked_prisoners(&self) -> usize {
        self.prisoners.len()
    }
}
