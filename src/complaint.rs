// This is basically defining the structure for the messages passed between the prisoner and
// guard threads.
pub struct Complaint {
    pub need: String,
    pub ammount: i8,
    pub prisoner_name: String
}

impl Complaint {
    pub fn new(need: &str, ammount: i8, prisoner_name: String) -> Complaint {
        Complaint {
            need: need.to_string(),
            ammount: ammount,
            prisoner_name: prisoner_name
        }
    }
}
