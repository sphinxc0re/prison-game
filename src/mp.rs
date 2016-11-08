use std::sync::mpsc::Sender;



#[derive(Debug)]
pub enum Message {
    Dead { prisoner_name: String },
    Alive { prisoner_name: String },
    Treatment { need: String, amount: i8 },
    Complaint { need: String, amount: i8 }
}

#[derive(Debug)]
pub struct Envelope {
    pub message: Message,
    pub return_sender: Sender<Envelope>
}

impl Envelope {
    pub fn new(message: Message, return_sender: Sender<Envelope>) -> Envelope {
        Envelope {
            message: message,
            return_sender: return_sender
        }
    }
}
