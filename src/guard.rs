use complaint::Complaint;
use std::sync::mpsc::channel;
use std::sync::mpsc::Receiver;
use std::collections::HashMap;
use std::sync::mpsc::Sender;



/// A simple type definition for consistency
pub type GuardVec = Vec<Guard>;

pub struct Guard {
    /// The sender of the channel, typed
    complaint_sender: Sender<Complaint>,
    /// The receiver of the channel, typed
    complaint_receiver: Receiver<Complaint>,
    /// The need, the guard is able to satisfy
    pub need: String,
    /// A map to keep track of the needs of the prisoners
    complaint_stats: HashMap< String, i8>
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
            need: need.to_string(),
            complaint_stats: HashMap::new()
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
    pub fn get_complaint_sender(&self) -> Sender<Complaint> {
        self.complaint_sender.clone()
    }

    /// Waits for a new complaint to be sent and returns it
    pub fn wait_for_and_receive_complaint(&self) -> Option<Complaint> {
        self.complaint_receiver.recv().ok()
    }

    pub fn track_complaint(&mut self, complaint: &Complaint) -> i8 {
        let prisoner_name = complaint.prisoner_name.clone();
        let ammount = complaint.ammount.clone();
        let original_ammount = self.complaint_stats.entry(prisoner_name).or_insert(0);
        *original_ammount += ammount;
        *original_ammount
    }
}
