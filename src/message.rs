use std::sync::mpsc::Sender;



#[derive(Debug)]
pub enum Message {
    Kill,
    NoAction,
    Complain(String, i8, String, Sender<Message>)
}
