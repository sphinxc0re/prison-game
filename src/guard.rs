use std::sync::mpsc::channel;
use std::sync::mpsc::Sender;
use std::sync::mpsc::Receiver;
use complaint::Complaint;



pub struct Guard {
    complaint_sender: Sender<Complaint>,
    pub complaint_receiver: Receiver<Complaint>,
    pub need: String
}


impl Guard {
    pub fn new(need: &str) -> Guard {
        let (snd, rec) = channel();
        Guard {
            complaint_sender: snd,
            complaint_receiver: rec,
            need: need.to_string()
        }
    }

    pub fn get_complaint_sender(&self) -> Sender<Complaint> {
        self.complaint_sender.clone()
    }
}

pub type GuardVec = Vec<Guard>;
