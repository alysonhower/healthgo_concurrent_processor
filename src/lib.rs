mod models;

use anyhow::{Context, Result};
pub use models::*;
use rusqlite::Connection;

// Starts the message processing system
// - Creates and initializes workers
// - Processes messages from the database
// - Handles inter-worker communication
pub async fn start(db_path: &str, verbose: bool) -> anyhow::Result<()> {
    let mut manager = Manager::new();
    manager.setup(db_path, verbose).await
}

// Prints database information for debugging purposes
// - Shows table schema
// - Displays sample message data
pub async fn print_db_info(db_path: &str) -> Result<()> {
    // Open database connection
    let conn = Connection::open(db_path)
        .with_context(|| format!("Failed to open database at {}", db_path))?;

    // Print table schema
    let mut stmt =
        conn.prepare("SELECT sql FROM sqlite_master WHERE type='table' AND name='messages_table'")?;
    let schema: String = stmt.query_row([], |row| row.get(0))?;
    println!("Table schema:\n{}\n", schema);

    // Print sample message data
    let mut stmt = conn.prepare("SELECT * FROM messages_table LIMIT 1")?;
    let row: (i32, String) = stmt.query_row([], |row| Ok((row.get(0)?, row.get(1)?)))?;
    println!("Sample row:\nID: {}\nData: {}\n", row.0, row.1);

    Ok(())
}
