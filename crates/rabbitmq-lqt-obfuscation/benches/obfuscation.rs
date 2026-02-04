// Copyright (C) 2025-2026 Michael S. Klishin and Contributors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use criterion::{BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};
use rabbitmq_lqt_obfuscation::LogObfuscator;
use std::fs::{File, metadata, read_dir};
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

fn get_log_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("benches/fixtures")
}

fn get_log_files() -> Vec<(String, PathBuf)> {
    let log_dir = get_log_dir();
    let mut files = Vec::new();

    if let Ok(entries) = read_dir(&log_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().is_some_and(|ext| ext == "log") {
                if let Some(name) = path.file_name() {
                    files.push((name.to_string_lossy().to_string(), path));
                }
            }
        }
    }

    files.sort_by(|a, b| a.0.cmp(&b.0));
    files
}

fn bench_obfuscate_files(c: &mut Criterion) {
    let mut group = c.benchmark_group("obfuscate_file");
    group.sample_size(10);

    for (name, path) in get_log_files() {
        let file_size = metadata(&path).map(|m| m.len()).unwrap_or(0);

        group.throughput(Throughput::Bytes(file_size));
        group.bench_with_input(BenchmarkId::from_parameter(&name), &path, |b, path| {
            b.iter(|| {
                let file = File::open(path).unwrap();
                let reader = BufReader::new(file);
                let mut obfuscator = LogObfuscator::new();
                let mut line_count = 0usize;

                for line_result in reader.lines() {
                    let line = line_result.unwrap();
                    let _ = obfuscator.obfuscate_line(&line);
                    line_count += 1;
                }

                line_count
            });
        });
    }

    group.finish();
}

fn bench_obfuscate_combined(c: &mut Criterion) {
    let mut group = c.benchmark_group("obfuscate_combined");
    group.sample_size(10);

    let files = get_log_files();
    if files.is_empty() {
        eprintln!("Warning: No log files found for benchmark");
        return;
    }

    let total_size: u64 = files
        .iter()
        .map(|(_, p)| metadata(p).map(|m| m.len()).unwrap_or(0))
        .sum();

    group.throughput(Throughput::Bytes(total_size));
    group.bench_function("all_files", |b| {
        b.iter(|| {
            let mut obfuscator = LogObfuscator::new();
            let mut total_lines = 0usize;

            for (_, path) in &files {
                let file = File::open(path).unwrap();
                let reader = BufReader::new(file);

                for line_result in reader.lines() {
                    let line = line_result.unwrap();
                    let _ = obfuscator.obfuscate_line(&line);
                    total_lines += 1;
                }
            }

            total_lines
        });
    });

    group.finish();
}

criterion_group!(benches, bench_obfuscate_files, bench_obfuscate_combined);
criterion_main!(benches);
