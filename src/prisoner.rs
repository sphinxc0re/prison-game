use mp::Message;
use mp::Envelope;
use guard::Guard;
use std::collections::HashMap;
use std::sync::mpsc::Sender;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::channel;




/// A simple type definition for consistency
pub type PrisonerVec = Vec<Prisoner>;

pub struct Prisoner {
    /// The name of the prisoner
    pub name: String,
    /// A hashmap mapping a need to the respective sender object of the guard
    guard_map: HashMap<String, Sender<Envelope>>,
    /// The sender channel of the prisoner, typed
    sender: Sender<Envelope>,
    /// The receiver channel of the prisoner, typed
    receiver: Receiver<Envelope>,
    /// The statistics of the prisoners needs
    need_map: HashMap<String, i8>
}

impl Prisoner {
    /// Sends a complaint to the respective guard
    pub fn complain(&self, complaint: Message) {
        match complaint {
            Message::Complaint { ref need, ref amount } => {
                let message = Message::Complaint { need: need.clone(), amount: amount.clone() };
                let envelope = Envelope::new(message, self.get_sender());
                self.guard_map[need].send(envelope).unwrap();
            },
            other => panic!("Prisoner is unable to send message of type: {:?}", other)
        }
    }

    /// Adds a guard to the guard map
    pub fn add_guard(&mut self, guard: &Guard) {
        self.guard_map.insert(guard.need.clone(), guard.get_sender());
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
            Self::new(name.as_str())
        }).collect()
    }

    pub fn receive_message(&self) -> Option<Envelope> {
        self.receiver.recv().ok()
    }

    /// Returns a copy of the message sender
    pub fn get_sender(&self) -> Sender<Envelope> {
        self.sender.clone()
    }

    pub fn broadcast_dead(&self) {
        for ref mut guard in self.guard_map.values().into_iter() {
            let envelope = Envelope::new(Message::Dead { prisoner_name: self.name.clone() }, self.get_sender());
            guard.send(envelope).expect("Message could not be sent");
        }
    }

    pub fn broadcast_alive(&self) {
        for ref mut guard in self.guard_map.values().into_iter() {
            let envelope = Envelope::new(Message::Alive { prisoner_name: self.name.clone() }, self.get_sender());
            guard.send(envelope).expect("Message could not be sent");
        }
    }

    pub fn track_need(&mut self, need: &String, amount: i8) -> i8 {
        let need_amount = self.need_map.entry((*need).clone()).or_insert(0);
        *need_amount += amount;
        *need_amount
    }

    /// Constructs a new `Prisoner`
    ///
    /// # Examples
    ///
    /// let prisoner = Prisoner::new("Jacob");
    pub fn new(name: &str) -> Prisoner {
        let (snd, rec) = channel();
        Prisoner {
            name: name.to_string(),
            guard_map: HashMap::new(),
            need_map: HashMap::new(),
            sender: snd,
            receiver: rec
        }
    }
}
