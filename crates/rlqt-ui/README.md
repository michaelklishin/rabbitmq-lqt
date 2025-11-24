# RabbitMQ Log Query Tools - Web UI

A modern web interface for querying and analyzing RabbitMQ log files

## Building

### Backend (Rust)

```bash
cargo build --package rlqt-ui
```

### Frontend (React)

```bash
cd crates/rlqt-ui/frontend
npm install
npm run build
```

## Running

First, ensure you have a parsed SQLite database (created using `rabbitmq-lqt logs parse`)

Then start the web server:

```bash
cargo run --package rlqt-ui -- web serve --input-db-file-path /path/to/logs.db
```

Or using the built binary:

```bash
./target/debug/rlqt-ui web serve --input-db-file-path /path/to/logs.db
```

The web UI will be available at http://127.0.0.1:15692

## Options

- `--input-db-file-path`: Path to the SQLite database file (required)
- `--host`: Host address to bind to (default: 127.0.0.1)
- `--port` / `-p`: Port to listen on (default: 15692)

## Development

To run the frontend in development mode with hot reloading:

```bash
cd crates/rlqt-ui/frontend
npm run dev
```

This will start a Vite dev server on port 5173 that proxies API requests to the backend
