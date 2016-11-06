use mp::Message;
use mp::Envelope;
use std::sync::mpsc::channel;
use std::sync::mpsc::Receiver;
use std::collections::HashMap;
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
    prisoner_need_stats: HashMap<String, i8>
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
            prisoner_need_stats: HashMap::new()
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

    /// track a prisoners stats per need
    pub fn track_complaint(&mut self, complaint: &Message) -> i8 {
        match complaint {
            &Message::Complain(_, ref ammount, ref prisoner_name) => {
                let original_ammount = self.prisoner_need_stats.entry((*prisoner_name).clone()).or_insert(0);
                *original_ammount += *ammount;
                *original_ammount
            },
            _ => unreachable!()
        }
    }

    pub fn untrack_prisoner(&mut self, name: &String) {
        self.prisoner_need_stats.remove(name);
    }

    pub fn track_prisoner(&mut self, name: &String) {
        self.prisoner_need_stats.entry((*name).clone()).or_insert(0);
    }

    pub fn tracked_prisoners(&self) -> usize {
        self.prisoner_need_stats.len()
    }
}
