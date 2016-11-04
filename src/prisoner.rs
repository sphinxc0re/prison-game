use message::Message;
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
    guard_map: HashMap<String, Sender<Message>>,
    /// The sender of the channel, typed
    sender: Sender<Message>,
    /// The receiver of the channel, typed
    receiver: Receiver<Message>,
}

impl Prisoner {
    /// Sends a complaint to the respective guard
    pub fn complain(&self, complaint: Message) {
        match complaint {
            Message::Complain(ref need, ref ammount, ref prisoner_name, ref sender) => {
                let actual_complaint = Message::Complain(need.clone(), ammount.clone(), prisoner_name.clone(), sender.clone());
                self.guard_map[need].send(actual_complaint).unwrap();
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

    pub fn wait_for_and_receive_message(&self) -> Option<Message> {
        self.receiver.recv().ok()
    }

    /// Returns a copy of the message sender
    pub fn get_sender(&self) -> Sender<Message> {
        self.sender.clone()
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
            sender: snd,
            receiver: rec
        }
    }
}
