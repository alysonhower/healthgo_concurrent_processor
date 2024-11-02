use anyhow::Result;
use clap::Parser;
use healthgo_concurrent_processor::{print_db_info, start};

// Command line arguments for the message processor
#[derive(Parser)]
#[command(
    author = "Alysonhower Veras Vieira",
    version,
    about = "A message processing system that handles worker communication and message routing.",
    long_about = None
)]
struct Args {
    #[arg(
        short,
        long,
        help = "Path to the SQLite database file containing messages.",
        long_help = "Path to a SQLite database file containing the messages table.\n\
                    The table must have:\n\
                    - 'id' column (INTEGER PRIMARY KEY)\n\
                    - 'data' column (TEXT) containing JSON with fields:\n\
                      * worker: Worker ID (1-5)\n\
                      * message: Message content\n\
                      * interval: Processing delay in ms\n\
                      * destination_worker: Target worker ID (1-5)"
    )]
    database: String,

    #[arg(
        short,
        long,
        help = "Enable verbose output mode.",
        long_help = "When enabled, prints detailed information including:\n\
                    - Database schema\n\
                    - Sample message data\n\
                    - Message processing intervals\n\
                    - Worker routing details"
    )]
    verbose: bool,
}

// Entry point of the message processor
// Sets up and runs the message processing system based on command line arguments
#[tokio::main]
async fn main() -> Result<()> {
    // Parse command line arguments
    let args = Args::parse();

    // If verbose mode is enabled, print database information before processing
    if args.verbose {
        print_db_info(&args.database).await?;
    }

    // Start the message processing system
    // This initializes workers, processes messages, and handles communication
    start(&args.database, args.verbose).await?;

    Ok(())
}
