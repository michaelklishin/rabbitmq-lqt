# RabbitMQ Log Querying Tools

RabbitMQ Log Querying Tools (RLQT), as the name suggests, are a set of tools designed for parsing and annotating
RabbitMQ log files for more efficient analysis.

This tool is intended for local use, including offline or air gapped environments. Its inputs are RabbitMQ log files
and its annotated data store is entirely local.


## Project Maturity

This tool is is young and should be considered an emerging project
that will evolve over time, for example, by adding more annotators and
improving the querying capabilities.


## Binary Releases

Binary releases are available [on the Releases page](https://github.com/michaelklishin/rabbitmq-lqt/releases).


## Usage

### CLI Interface Help

Start with

```shell
rabbitmq-lqt help
```

All command groups and individual commands support `--help`:

```shell
rabbitmq-lqt logs --help

rabbitmq-lqt web --help

rabbitmq-lqt logs parse --help

rabbitmq-lqt logs query --help
```


### Parsing and Annotating Log files

This tool takes on a group of log files, one or more per node, using the standard RabbitMQ log file
naming convention where the node name is included into the file (e.g. `rabbit@hostname1.eng.megacorp.local.log`),
parses them, then annotates the log entries and produces a database file
for querying.

To parse and annotate a set of files, use `rabbitmq-lqt logs parse`

```shell
rm -f /tmp/log_set_abc.rlqt

rabbitmq-lqt logs parse --input-log-file-path /path/to/rabbit@node1.log \
                        --input-log-file-path /path/to/rabbit@node2.log \
                        --input-log-file-path /path/to/rabbit@node3.log \
                        --output-db-file-path /tmp/log_set_abc.rlqt
```

### Querying Annotated Data

`rabbitmq-lqt logs query` is a command for querying the results:

```shell
# show error messages within a date range
rabbitmq-lqt logs query --input-db-file-path /tmp/log_set_abc.rlqt \
                        --since-time "5 days ago" --to-time "2 days ago" \
                        --severity error

# show up to 2000 most recent messages related to Raft leader elections
rabbitmq-lqt logs query --input-db-file-path /tmp/log_set_abc.rlqt \
                        --label raft --label election \
                        # combined the above labels using a logical "AND"
                        --matching-all-labels \
                        --limit 2000

# show all messages related to feature flags
rabbitmq-lqt logs query --input-db-file-path /tmp/log_set_abc.rlqt \
                        --subsystem feature_flags
```

### Web UI for Querying

Besides `logs query` on the command line, a Web UI can be used to query the parsed
and annotated log entries:

```shell
rabbitmq-lqt web serve -i /tmp/log_set_abc.rlqt
# => (elided for brevity)
# => [INFO][rlqt_ui::server] Server listening on http://127.0.0.1:15692
```

then navigate to `http://127.0.0.1:15692`.


### Working with Log File Updates

When log files change, the annotation database must be deleted and re-created using the `logs parse` command.
Incremental updates are intentionally not supported.


## Large Log File support

This tool supports log files up to million lines long. Each input file is parsed in parallel.
Result annotation is also optimized for multi-core CPUs.

A 1M log file can be parsed and annotated in about a minute on an M1 MacBook Pro from late 2021,
and three 1M log files can be parsed and annotated in less than two minutes.

Note that as the number of implemented annotations grows, the annotation process will inevitably become slower
because annotation includes an inherent O(n) operation (all annotators must be tried/traversed).


## Subprojects

 * `crates/rlqt-lib` is a library that implements the parser and annotators
 * `crates/rlqt-cli` is the `rabbitmq-lqt` command line tool
 * `crates/rlqt-ui` is a Web UI


## License

This project is double licensed under the MIT License and the Apache License, Version 2.0.

See `LICENSE-APACHE` and `LICENSE-MIT` for details.

SPDX-License-Identifier: Apache-2.0 OR MIT
