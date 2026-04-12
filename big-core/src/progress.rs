use std::sync::mpsc::{Receiver, Sender, channel};

/// Progress events emitted by long-running operations (pack/extract).
#[derive(Debug, Clone)]
pub enum Progress {
    Started { job: String },
    Percent(u8),
    Message(String),
    Completed,
    Error(String),
}

pub type ProgressSender = Sender<Progress>;
pub type ProgressReceiver = Receiver<Progress>;

/// Create a new progress channel pair.
pub fn progress_channel() -> (ProgressSender, ProgressReceiver) {
    channel()
}
