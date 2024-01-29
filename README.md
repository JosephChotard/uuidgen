# UUID Generator

A simple CLI application to generate UUIDs using Rust and `clap`.

## Prerequisites

- Rust: Install Rust using [rustup](https://rustup.rs/)

## Building

1. Clone the repository:

   ```
   git clone https://github.com/yourusername/uuid-generator.git
   cd uuid-generator
   ```

2. Build the project:

   ```
   cargo build --release
   ```

   The compiled binary will be available at `target/release/uuid-generator`.

## Usage

```
USAGE:
    uuid-generator [OPTIONS]

OPTIONS:
    -n, --num-uuids <NUM_UUIDS>
            Number of UUIDs to generate [default: 1]

    -h, --help
            Prints help information
```

### Examples

1. Generate a single UUID (default):

   ```
   ./target/release/uuid-generator
   ```

2. Generate 5 UUIDs:

   ```
   ./target/release/uuid-generator -n 5
   ```