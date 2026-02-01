# Instructions for AI Agents

## Overview

This repository contains RabbitMQ Log Querying Tools (RLQT), a set of tools designed for parsing and annotating
RabbitMQ log files for more efficient analysis.

## Repository Layout

This is a Rust workspace managed by `cargo`. The repository layout is as follows:

 * `Cargo.toml`: the workspace manifest file
 * `crates/rlqt-cli`: CLI tool for parsing and querying RabbitMQ log files
 * `crates/rlqt-lib`: a core library for log parsing, annotation, metadata management, and database operations

### `rabbitmq-lqt`, a.k.a. the `rlqt-cli` crate

This crate contains `rabbitmq-lqt`, a CLI tool, including its CLI parser,
core execution parts, error handling, configuration, integration tests, and so on.

 * `main.rs` contains the entry point
 * the `cli` module at `crates/rlqt-cli/src/cli.rs` that defines a [`clap`](https://crates.io/crates/clap)-based CLI parser
 * the `commands` module at `crates/rlqt-cli/src/commands.rs` implements command handlers
 * the `core` module at `crates/rlqt-cli/src/core.rs` contains core business logic
 * the `errors` module at `crates/rlqt-cli/src/errors.rs` defines CLI tool errors
 * the `output` module at `crates/rlqt-cli/src/output.rs` handles formatting and display of query results

`rlqt-lib` provides the key building blocks used by `rlqt-cli`.

### The `rlqt-lib` crate

This library is the heart of the codebase.

Key modules in the `rlqt-lib` crate are:

 * `crates/rlqt-lib/src/lib.rs` is the library entry point
 * the `parser` module at `crates/rlqt-lib/src/parser.rs`: parsing of RabbitMQ log files
 * the `severity` module at `crates/rlqt-lib/src/severity.rs`: log severity levels
 * the `errors` module at `crates/rlqt-lib/src/errors.rs` defines library errors
 * the `constants` module at `crates/rlqt-lib/src/constants.rs`: constant values used across the library
 * the `metadata` module at `crates/rlqt-lib/src/metadata/`: log entry annotations (classification, labelling)
 * the `rel_db` module at `crates/rlqt-lib/src/rel_db/`: relational database models and operations

### The `rlqt-obfuscation` crate

This crate obfuscates log file entries, namely:

 * Usernames
 * Node names, hostnames, IP addresses
 * Virtual host names

 It is exposed as a CLI command, `logs obfuscate`.

 Thanks to this crate, real world files can be safely used for development
 and even added as new fixture files.


## Key Concepts

RLQT parses RabbitMQ log files and stores them in an embedded database ([DuckDB](https://duckdb.org/docs/stable/)) for efficient querying.
Each log entry is annotated with metadata including:

 * **Subsystems**: the RabbitMQ subsystem the log entry relates to (e.g., connections, queues, virtual hosts)
 * **Labels**: semantic tags applied to log entries (e.g., access control, auto-delete, federation)
 * **Documentation URLs**: links to relevant RabbitMQ documentation
 * **Known Issue/Resolution URLs**: links to related GitHub issues, pull requests or commits

The tool supports querying by severity, time range, subsystem, labels, and text search.

## Build System

 * To build the workspace, run `cargo build --all`
 * To run the `rabbitmq-lqt` CLI, run `cargo run --package rlqt-cli --bin rabbitmq-lqt`
 * To run the tests, run `cargo nextest run --all`
 * To run benchmarks, run `cargo bench` from the `crates/rlqt-lib` directory

## Versioning

The version in `crates/rlqt-ui/frontend/package.json` must match the Rust workspace version in `Cargo.toml`. Keep them in sync when bumping versions.

## Target Rust Version

 * This tool targets cutting edge Rust (currently `1.91.0`)

## Key Dependencies

 * `nom` for parsing log files
 * `tokio` the asynchronous runtime
 * `sea_orm` for storing and querying of tabular (relational) data
 * `chrono` and `chrono-english` for datetime parsing and natural language date expressions
 * `clap` for CLI argument parsing
 * `tabled` for tabular output on the command line
 * `regex` for pattern matching in log messages

## Rust Code Style

 * Prefer short names like `Display` or `fmt::Display` with a `use` statement over fully-qualified names like `std::fmt::Display`
 * Never use function-local `use` statements (imports)
 * Add tests to the modules under `tests`, never in the implementation files
 * At the end of each task, run `cargo fmt --all`
 * At the end of each task, run `cargo clippy --all` and fix any warnings it might emit

## Tests

Never use made-up (fabricated) log strings, only real log entries from RabbitMQ log files,
RabbitMQ source code or documentation guides.

For false positive tests, use real log entries from other tests, not dummy fabricated strings.

## Benchmarks

The `rlqt-lib` crate includes comprehensive benchmarks for the parse-annotate-store pipeline
and some individual parts such as log entry annotations.

## Comments

 * Only add very important comments, both in tests and in the implementation

## Git Instructions

 * Never add yourself to the list of commit co-authors
 * Never mention yourself in commit messages in any way (no "Generated by", no AI tool links, etc)

## Style Guide

 * Never add full stops to Markdown list items

## Refactoring Ideas To Never Suggest (or Try)

 * Using macros to refactor `crates/rlqt-lib/src/entry_metadata/labels.rs`

## After Completing a Task

### Iterative Reviews

After completing a task, perform up to twenty iterative reviews of your changes.
In every iteration, look for meaningful improvements that were missed, for gaps in test coverage,
and for deviations from the instructions in this file.

If no meaningful improvements are found for three iterations in a row,
report it and stop iterating.
