use std::sync::mpsc::Sender;



#[derive(Debug)]
pub enum Message {
    Kill,
    NoAction,
    Dead(String),
    Alive(String),
    Complain { need: String, ammount: i8, prisoner_name: String }
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
