# conc-url-pinger

A simple Rust application that pings multiple URLs concurrently.

## Features

- Asynchronous HTTP requests with Tokio
- Concurrency limiting using semaphore
- Basic error handling

## Usage

```bash
cargo run
```

You can edit the URLs and concurrency limit in the `main.rs` file.

## Dependencies

- `tokio` - Async runtime
- `reqwest` - HTTP client

## License

MIT
