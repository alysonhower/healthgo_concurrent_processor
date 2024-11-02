use super::message::MessageState;
use tokio::sync::mpsc::Sender;

// Worker represents a message processing unit
pub struct Worker {
    pub id: i32,
    pub sender: Sender<MessageState>,
    pub last_sender_id: i32,
    pub last_message: String,
}

impl Worker {
    pub fn new(id: i32, sender: Sender<MessageState>) -> Self {
        Self {
            id,
            sender,
            last_sender_id: 0,
            last_message: String::new(),
        }
    }

    // Updates worker's state with information from received message
    pub fn update_state(&mut self, from_worker: i32, message: String) {
        self.last_sender_id = from_worker;
        self.last_message = message;
    }
}
