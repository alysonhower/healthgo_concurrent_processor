# HealthGo Code Challenge - Concurrent Processor

This repository contains my solution to HealthGo's code challenge, implementing a concurrent message processing system in Rust.

## Challenge Overview

The challenge required building a system that demonstrates:
- Concurrent processing capabilities
- Inter-process/thread communication
- JSON data handling with SQLite integration
- Clean code architecture and documentation

## Solution Implementation

I implemented a message processing system that:
- Manages 5 concurrent workers using Tokio async runtime
- Handles inter-worker communication through MPSC channels
- Processes JSON-formatted messages stored in SQLite
- Maintains message ordering and timing requirements
- Provides comprehensive test coverage

### Technical Highlights

- **Concurrency**: Uses Tokio for async runtime and MPSC channels for safe message passing
- **Thread Safety**: Implements Arc<Mutex> for shared state management
- **Database Integration**: SQLite with proper connection handling and prepared statements
- **Error Handling**: Comprehensive error handling using anyhow
- **Testing**: Full integration test suite covering various scenarios

## Getting Started

### Prerequisites

- MSVC (for Windows users)
- Rustup (for installing Rust and Cargo)

### Building and Running

```bash
# Clone the repository
git clone <repository-url>
cd healthgo-concurrent-processor

# Build the project
cargo build --release

# Run with a database file
healthgo-concurrent-processor --database path/to/database.db

# Enable verbose output
healthgo-concurrent-processor --database path/to/database.db --verbose
```

### Running Tests

```bash
# Run all tests
cargo test

# Run integration tests specifically
cargo test --test integration_test
```

## Project Structure

```
healthgo-concurrent-processor/
├── src/
│   ├── models/
│   │   ├── data.rs       # Database message structure
│   │   ├── message.rs    # Message state handling
│   │   ├── worker.rs     # Worker implementation
│   │   └── router.rs     # Concurrent message routing
│   ├── lib.rs            # Core functionality
│   └── main.rs           # CLI interface
├── tests/
│   └── integration_test.rs
└── Cargo.toml
```

## Test Coverage

The integration tests verify:
- Basic message processing functionality
- Worker-to-worker communication
- Message timing and intervals
- Error handling for invalid inputs
- Multi-step message chains

## Error Handling

The system provides clear error messages for:
- Database connection issues
- Invalid JSON data
- Invalid worker IDs
- Missing or corrupt database files

## Design Decisions

1. **Tokio Async Runtime**: Chosen for its robust concurrency support and async/await syntax
2. **MPSC Channels**: Used for safe communication between workers
3. **Arc<Mutex>**: Ensures thread-safe access to shared worker state

## Future Improvements

While the current implementation meets all challenge requirements, potential improvements could include:
- Metrics collection for worker performance
- Dynamic worker pool sizing
- Message persistence and recovery
- Real-time monitoring interface
