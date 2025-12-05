# RabbitMQ Log Query Tools - Web UI

This is a Web equivalent of the `logs query` command.

It includes a React-based frontend that's embedded into the `rabbitmq-lqt` binary
at compile time.

## Running

First, make sure to parse and annotate a set of log files using `rabbitmq-lqt logs parse`.
It will produce a database file that is used by this UI as the data source.

Then start an API server:

```bash
rabbitmq-lqt web serve --input-db-file-path /path/to/logs.db
```

The web UI will be available at http://127.0.0.1:15692

## Building

### Full Build

The frontend is automatically rebuilt by `cargo build` (using a custom `build.rs`) when relevant files change:

```bash
cargo build --package rlqt-cli
```

### Frontend Only

To rebuild just the frontend assets:

```bash
cd crates/rlqt-ui/frontend
npm install
npm run build
```

## Development

To run the frontend in development mode with hot reloading:

```bash
cd crates/rlqt-ui/frontend
npm run dev
```

This will start a Vite dev server on port 5173 that proxies API requests to the backend.
