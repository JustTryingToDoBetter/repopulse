# RepoPulse

RepoPulse is a Rust-based command-line tool that analyzes public GitHub repositories using the GitHub REST API.

## Features

- Parses GitHub repository URLs
- Fetches live repository metadata from GitHub
- Calculates a simple repository health score
- Uses async HTTP requests with Tokio and Reqwest
- Handles API errors cleanly
- Includes unit tests for core logic

## Tech Stack

- Rust
- Tokio
- Reqwest
- Serde
- Clap
- GitHub REST API

## Example

```bash
cargo run -- https://github.com/rust-lang/rust