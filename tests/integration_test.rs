use anyhow::Result;
use healthgo_concurrent_processor::{start, Data};
use rusqlite::Connection;
use tempfile::NamedTempFile;

async fn setup_test_db(messages: Vec<Data>) -> Result<NamedTempFile> {
    let temp_db = NamedTempFile::new()?;
    let conn = Connection::open(temp_db.path())?;

    conn.execute(
        "CREATE TABLE messages_table (
            id INTEGER PRIMARY KEY,
            data TEXT NOT NULL
        )",
        [],
    )?;

    let mut stmt = conn.prepare("INSERT INTO messages_table (data) VALUES (?)")?;
    for message in messages {
        let json_data = serde_json::to_string(&message)?;
        stmt.execute([json_data])?;
    }

    Ok(temp_db)
}

#[tokio::test]
async fn basic_message_processing() -> Result<()> {
    let messages = vec![
        Data {
            worker: 1,
            message: "Hello".to_string(),
            interval: 100,
            destination_worker: 2,
        },
        Data {
            worker: 2,
            message: "World".to_string(),
            interval: 100,
            destination_worker: 3,
        },
    ];

    let temp_db = setup_test_db(messages).await?;
    start(temp_db.path().to_str().unwrap(), false).await?;
    Ok(())
}

#[tokio::test]
async fn worker_routing() -> Result<()> {
    let messages = vec![
        Data {
            worker: 1,
            message: "Message 1".to_string(),
            interval: 50,
            destination_worker: 2,
        },
        Data {
            worker: 2,
            message: "Message 2".to_string(),
            interval: 50,
            destination_worker: 1,
        },
        Data {
            worker: 1,
            message: "Message 3".to_string(),
            interval: 50,
            destination_worker: 3,
        },
    ];

    let temp_db = setup_test_db(messages).await?;
    start(temp_db.path().to_str().unwrap(), true).await?;
    Ok(())
}

#[tokio::test]
async fn timing_and_intervals() -> Result<()> {
    let messages = vec![
        Data {
            worker: 1,
            message: "Fast".to_string(),
            interval: 50,
            destination_worker: 2,
        },
        Data {
            worker: 2,
            message: "Slow".to_string(),
            interval: 150,
            destination_worker: 3,
        },
        Data {
            worker: 3,
            message: "Medium".to_string(),
            interval: 100,
            destination_worker: 1,
        },
    ];

    let temp_db = setup_test_db(messages).await?;
    start(temp_db.path().to_str().unwrap(), true).await?;
    Ok(())
}

#[tokio::test]
async fn invalid_worker_ids() -> Result<()> {
    let messages = vec![Data {
        worker: 6,
        message: "Invalid".to_string(),
        interval: 100,
        destination_worker: 1,
    }];

    let temp_db = setup_test_db(messages).await?;
    start(temp_db.path().to_str().unwrap(), true).await?;
    Ok(())
}

#[tokio::test]
async fn message_chain() -> Result<()> {
    let messages = vec![
        Data {
            worker: 1,
            message: "Start".to_string(),
            interval: 50,
            destination_worker: 2,
        },
        Data {
            worker: 2,
            message: "Middle".to_string(),
            interval: 50,
            destination_worker: 3,
        },
        Data {
            worker: 3,
            message: "End".to_string(),
            interval: 50,
            destination_worker: 1,
        },
    ];

    let temp_db = setup_test_db(messages).await?;
    start(temp_db.path().to_str().unwrap(), true).await?;
    Ok(())
}
