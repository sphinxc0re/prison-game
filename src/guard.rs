use complaint::Complaint;
use std::sync::mpsc::channel;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;



/// A simple type definition for consistency
pub type GuardVec = Vec<Guard>;

pub struct Guard {
    /// The sender of the channel, typed
    complaint_sender: Sender<Complaint>,
    /// The receiver of the channel, typed
    complaint_receiver: Receiver<Complaint>,
    /// The need, the guard is able to satisfy
    pub need: String
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
            complaint_sender: snd,
            complaint_receiver: rec,
            need: need.to_string()
        }
    }

    /// Constructs a new `GuardVec`
    ///
    /// # Examples
    /// ```
    /// let need_vec = vec![
    ///     "health",
    ///     "food",
    ///     "freedom",
    /// ];
    ///
    /// let guard_vec = Guard::new_vec(need_vec);
    /// ```
    pub fn new_vec(need_vec: Vec<String>) -> GuardVec {
        need_vec.into_iter().map(|need| {
            Guard::new(need.as_str())
        }).collect()
    }

    /// Returns a copy of the complaint sender
    pub fn get_complaint_sender(&self) -> Sender<Complaint> {
        self.complaint_sender.clone()
    }

    /// Waits for a new complaint to be sent and returns it
    pub fn wait_for_and_receive_complaint(&self) -> Complaint {
        self.complaint_receiver.recv().unwrap()
    }
}
