use std::fmt::{Display, Formatter, Result};

use super::data::Data;

// Represents the current state of a message in the system
#[derive(Debug)]
pub struct MessageState {
    pub state: MessageInfo,
    pub timing: TimingInfo,
    pub routing: RoutingInfo,
}

// Contains the actual message content and history
#[derive(Debug)]
pub struct MessageInfo {
    pub content: String,
    pub last_heard: String,
}

// Tracks timing information for message processing
#[derive(Debug)]
pub struct TimingInfo {
    pub interval_ms: i64,
    pub total_elapsed_ms: i64,
}

// Contains routing information for message delivery
#[derive(Debug)]
pub struct RoutingInfo {
    pub sender_id: i32,
    pub recipient_id: i32,
    pub last_sender_id: i32,
}

// Converts raw data from database into message state
impl From<Data> for MessageState {
    fn from(data: Data) -> Self {
        MessageState {
            state: MessageInfo {
                content: data.message,
                last_heard: String::new(),
            },
            timing: TimingInfo {
                interval_ms: data.interval,
                total_elapsed_ms: data.interval,
            },
            routing: RoutingInfo {
                sender_id: data.worker,
                recipient_id: data.destination_worker,
                last_sender_id: 0,
            },
        }
    }
}

impl MessageState {
    pub fn display<'a>(&'a self, verbose: bool) -> MessageDisplay<'a> {
        MessageDisplay {
            message: self,
            verbose,
        }
    }
}

// Handles formatting of message state for display
pub struct MessageDisplay<'a> {
    message: &'a MessageState,
    verbose: bool,
}

impl<'a> Display for MessageDisplay<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        writeln!(f, "worker: {}", self.message.routing.sender_id)?;
        writeln!(
            f,
            "worker_listened: {}",
            self.message.routing.last_sender_id
        )?;
        writeln!(f, "message: \"{}\"", self.message.state.content)?;
        writeln!(f, "message_listened: \"{}\"", self.message.state.last_heard)?;

        if self.verbose {
            writeln!(f, "interval: {} ms", self.message.timing.interval_ms)?;
            writeln!(
                f,
                "accumulated_time: {} ms",
                self.message.timing.total_elapsed_ms
            )?;
            writeln!(
                f,
                "destination_worker: {}",
                self.message.routing.recipient_id
            )?;
        }
        writeln!(f)
    }
}
