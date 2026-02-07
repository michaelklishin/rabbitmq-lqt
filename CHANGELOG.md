# rabbitmq-lqt Change Log

## v0.20.0 (in development)

### Bug Fixes

 * Updated `logs query --label` to validate against an up-to-date list of labels
 * `logs query --label` now correctly matches `amqp10` for AMQP 1.0-related events (and not `amqp1_0`)
 * `logs query --subsystem` now validates against a few more subsystems: `amqp10`, `management`, `metrics`, `oauth2`
 * `logs query --label` now recognizes a few missing labels: `mnesia`, `oauth2`, `sql`
 * `logs ql`: two pipeline stages, `| tail N` and `| offset N`, were unintentionally ignored

### Enhancements

 * Experimental I/O stream parser and filtering (as opposed to the original file-based one)


## v0.19.0 (Feb 2, 2026)

### Internal Changes

 * Release infrastructure: adopt [`michaelklishin/rust-release-action`](https://github.com/michaelklishin/rust-release-action)


## v0.18.0 (Jan 25, 2026)

### Enhancements

 * QL now supports hashtag syntax for labels: `#tls`, `#connections`, `#timeouts`
 * QL now also supports negated labels with a syntax extension: `-#label` is a shorthand for `not #label`

   Example queries:

    * `#tls`: entries with the TLS label
    * `#connections and -#timeouts`: connection entries without timeouts
    * `:errors and #shovels`: errors related to shovels


## v0.17.0 (Dec 20, 2025)

### Enhancements

 * New QL preset: `:raft` (aliases: `:raft_and_quorum_queues`, `:quorum_queues`) for Raft, quorum queue,
   and Raft leader elections events
 * More label annotators, in particular for the entries typical of RabbitMQ 3.13.x
 * `logs parse` and `logs merge` now support compressed input files:
   `.log.gz`, `.log.xz`, `.tar.gz`, `.tgz`, `.tar.xz`, `.txz`


## v0.16.0 (Dec 14, 2025)

### Enhancements

 * New command: `logs ql` is the CLI equivalent of the QL (for "query language") tab in the UI
 * `web serve` now automatically opens server URL in the browser


## v0.15.0 (Dec 13, 2025)

### Enhancements

 * Introduce a QL (Query Language) for more flexible filtering and sorting.
   This includes both a CLI command and a new Web UI tab.

   Example queries:

    - `:errors` — all error logs using the errors preset
    - `@24h subsystem == "connections"` — connection logs from the last 24 hours
    - `:crashes | sort timestamp desc` — crashes preset, sorted newest first
    - `message contains "timeout" | limit 50` — messages containing "timeout", limited to 50
    - `@1h severity == "warning" or severity == "error"` — warnings or errors from the last hour
    - `labels any ["tls", "disconnects"]` — entries with TLS or disconnect labels

 * QL supports time ranges (`@1h`, `@24h`, `@7d`), presets (`:errors`, `:crashes`),
   field filters (`severity ==`, `subsystem ==`, `message contains`), boolean logic
   (`and`, `or`, `not`), grouping with parentheses, and pipeline stages (`| limit`, `| sort`)


## v0.14.0 (Dec 11, 2025)

### Enhancements

 * New command: `logs merge` merges additional log files into an existing database


## v0.13.0 (Dec 11, 2025)

### Bug Fixes

 * The parser did not handle Erlang/OTP SASL report headers, which can still be present
   in real world RabbitMQ logs in some cases

### Enhancements

 * `logs obfuscate` now obfuscates queue names, exchange names, stream names, and policy names
 * in addition, `logs obfuscate` now obfuscates queue and node names in Ra (Raft) member names
 * `logs obfuscate` now avoids double-obfuscation when processing already-obfuscated (or partially obfuscated) log files
 * An experimental preset subsystem. A preset is a collection of filters used together.
 * Web UI now includes the first preset: Errors and Exceptions

## v0.12.0 (Dec 6, 2025)

### Enhancements

 * A 60-65% performance improvement for `parse logs`
 * Annotations for close to 200 new log entry types
 * New subsystems, labels
 * Labels are now stored using a bit set, improving `parse logs` performance by 20-25% for
   several example log files of different sizes
 * Improved annotator test coverage for negative cases



## v0.11.0 (Dec 2, 2025)

### Bug Fixes

 * `web serve` failed when the tool wasn't run with `cargo run` (built from source)
 * Web UI assets were not included into the release binary

### Enhancements

 * `logs parse` now supports `--input-log-dir-path` which accepts a directory path instead of
   individual file paths
 * On the file set metadata page in the UI, the files are now sorted by their length
 * Web UI is now rebuilt (with `npm build`) by `cargo build`
 * Dependency updates
