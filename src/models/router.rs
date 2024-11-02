use anyhow::{Context, Result};
use rusqlite::{Connection, Error as SqliteError};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio::time::{sleep, Duration};

use super::data::Data;
use super::message::MessageState;
use super::worker::Worker;

// Manager handles the coordination of workers and message routing
pub struct Manager {
    // Thread-safe shared map of worker IDs to workers
    workers: Arc<tokio::sync::Mutex<HashMap<i32, Worker>>>,
    // Channel for collecting processed messages from all workers
    tx_main: Option<mpsc::Sender<MessageState>>,
    rx_main: mpsc::Receiver<MessageState>,
}

impl Manager {
    pub fn new() -> Self {
        // Create a channel with buffer size 100 for collecting processed messages
        let (tx_main, rx_main) = mpsc::channel::<MessageState>(100);
        let workers = Arc::new(tokio::sync::Mutex::new(HashMap::new()));

        Self {
            workers,
            tx_main: Some(tx_main),
            rx_main,
        }
    }

    pub async fn setup(&mut self, db_path: &str, verbose: bool) -> Result<()> {
        self.initialize_workers().await;
        self.start(db_path).await?;
        self.cleanup_and_print_messages(verbose).await;
        Ok(())
    }

    // Creates 5 workers and sets up their message channels
    async fn initialize_workers(&mut self) {
        let mut workers = self.workers.lock().await;
        for worker_id in 1..=5 {
            let (tx, mut rx) = mpsc::channel::<MessageState>(100);
            let tx_main = self.tx_main.as_ref().unwrap().clone();

            workers.insert(worker_id, Worker::new(worker_id, tx));

            // Spawn a task for each worker to forward messages to main channel
            tokio::spawn(async move {
                while let Some(msg) = rx.recv().await {
                    let _ = tx_main.send(msg).await;
                }
            });
        }
    }

    // Reads messages from SQLite and routes them between workers
    async fn start(&self, db_path: &str) -> Result<()> {
        let conn = Connection::open(db_path)
            .with_context(|| format!("Failed to open database at {}", db_path))?;

        let mut stmt = conn.prepare("SELECT id, data FROM messages_table ORDER BY id ASC")?;

        // Parse JSON messages from database
        let messages = stmt.query_map([], |row| {
            let data: String = row.get(1)?;
            match serde_json::from_str::<Data>(&data) {
                Ok(message_data) => Ok(message_data),
                Err(e) => {
                    println!("Error parsing JSON data: {}\nData: {}", e, data);
                    Err(SqliteError::InvalidQuery)
                }
            }
        })?;

        let mut total_elapsed = 0;

        // Process each message with specified intervals
        for message in messages {
            let message = message?;

            sleep(Duration::from_millis(message.interval as u64)).await;
            total_elapsed += message.interval;

            let mut workers = self.workers.lock().await;

            if let Some(worker) = workers.get_mut(&message.worker) {
                let mut msg_state = MessageState::from(message.clone());
                msg_state.routing.last_sender_id = worker.last_sender_id;
                msg_state.state.last_heard = worker.last_message.clone();
                msg_state.timing.total_elapsed_ms = total_elapsed;

                // Update destination worker's state
                if let Some(dest_worker) = workers.get_mut(&message.destination_worker) {
                    dest_worker.update_state(message.worker, message.message.clone());
                }

                // Send message through worker's channel
                if let Some(worker) = workers.get(&message.worker) {
                    let _ = worker.sender.send(msg_state).await;
                }
            }
        }

        Ok(())
    }

    // Cleans up resources and prints processed messages
    async fn cleanup_and_print_messages(&mut self, verbose: bool) {
        {
            let mut workers = self.workers.lock().await;
            workers.clear();
        }
        self.tx_main.take();
        while let Some(msg) = self.rx_main.recv().await {
            print!("{}", msg.display(verbose));
        }
    }
}
