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

use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use rayon::prelude::*;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use tempfile::TempDir;
use tokio::sync::mpsc;

use rlqt_lib::entry_metadata::annotate_entry;
use rlqt_lib::parser::{ParsedLogEntry, parse_log_file};
use rlqt_lib::rel_db::{
    create_database, create_database_for_bulk_import, finalize_bulk_import,
    node_log_entry::NodeLogEntry, post_insertion_operations,
};
use sea_orm::DatabaseConnection;

const PIPELINE_CHUNK_SIZE: usize = 25_000;

fn get_fixture_path(lines: usize) -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.pop();
    path.pop();
    path.push("benchmarks");
    path.push("fixtures");
    path.push(format!("rabbit@sunnyside_{}.log", lines));
    path
}

fn bench_parse_only(c: &mut Criterion) {
    let mut group = c.benchmark_group("parse_only");
    group.sample_size(10);

    for &lines in &[250_000, 500_000, 1_000_000, 2_000_000] {
        let fixture_path = get_fixture_path(lines);

        if !fixture_path.exists() {
            eprintln!("Warning: Fixture not found: {:?}", fixture_path);
            continue;
        }

        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{}K", lines / 1000)),
            &fixture_path,
            |b, path| {
                b.iter(|| {
                    let file = File::open(path).unwrap();
                    let reader = BufReader::new(file);
                    parse_log_file(reader).unwrap().entries
                });
            },
        );
    }

    group.finish();
}

fn bench_parse_and_annotate(c: &mut Criterion) {
    let mut group = c.benchmark_group("parse_and_annotate");
    group.sample_size(10);

    for &lines in &[250_000, 500_000, 1_000_000, 2_000_000] {
        let fixture_path = get_fixture_path(lines);

        if !fixture_path.exists() {
            eprintln!("Warning: Fixture not found: {:?}", fixture_path);
            continue;
        }

        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{}K", lines / 1000)),
            &fixture_path,
            |b, path| {
                b.iter(|| {
                    let file = File::open(path).unwrap();
                    let reader = BufReader::new(file);
                    let mut entries = parse_log_file(reader).unwrap().entries;

                    entries.par_iter_mut().for_each(|entry| {
                        annotate_entry(entry);
                    });

                    entries.sort_by_key(|e| e.sequence_id);

                    entries
                });
            },
        );
    }

    group.finish();
}

fn bench_full_pipeline(c: &mut Criterion) {
    let mut group = c.benchmark_group("full_pipeline");
    group.sample_size(10);

    for &lines in &[250_000, 500_000, 1_000_000] {
        let fixture_path = get_fixture_path(lines);

        if !fixture_path.exists() {
            eprintln!("Warning: Fixture not found: {:?}", fixture_path);
            continue;
        }

        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{}K", lines / 1000)),
            &fixture_path,
            |b, path| {
                b.to_async(tokio::runtime::Runtime::new().unwrap())
                    .iter(|| async {
                        let temp_dir = TempDir::new().unwrap();
                        let db_path = temp_dir.path().join("benchmark.db");
                        let db = create_database(&db_path).await.unwrap();

                        let file = File::open(path).unwrap();
                        let reader = BufReader::new(file);
                        let mut entries = parse_log_file(reader).unwrap().entries;

                        entries.par_iter_mut().for_each(|entry| {
                            annotate_entry(entry);
                        });

                        entries.sort_by_key(|e| e.sequence_id);

                        NodeLogEntry::insert_parsed_entries(&db, &entries, "rabbit@sunnyside")
                            .await
                            .unwrap();

                        post_insertion_operations(&db).await.unwrap();
                    });
            },
        );
    }

    group.finish();
}

async fn insert_chunk(
    db: &DatabaseConnection,
    chunk: Vec<ParsedLogEntry>,
    node: &str,
) -> Result<(), sea_orm::DbErr> {
    NodeLogEntry::insert_parsed_entries(db, &chunk, node).await
}

fn bench_pipelined_full(c: &mut Criterion) {
    let mut group = c.benchmark_group("pipelined_full");
    group.sample_size(10);

    for &lines in &[250_000, 500_000] {
        let fixture_path = get_fixture_path(lines);

        if !fixture_path.exists() {
            eprintln!("Warning: Fixture not found: {:?}", fixture_path);
            continue;
        }

        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{}K", lines / 1000)),
            &fixture_path,
            |b, path| {
                b.to_async(tokio::runtime::Runtime::new().unwrap())
                    .iter(|| async {
                        let temp_dir = TempDir::new().unwrap();
                        let db_path = temp_dir.path().join("benchmark.db");
                        let db = create_database(&db_path).await.unwrap();

                        let file = File::open(path).unwrap();
                        let reader = BufReader::new(file);
                        let entries = parse_log_file(reader).unwrap().entries;

                        let (tx, mut rx) = mpsc::channel::<Vec<ParsedLogEntry>>(4);

                        let db_clone = db.clone();
                        let insert_task = tokio::spawn(async move {
                            while let Some(chunk) = rx.recv().await {
                                insert_chunk(&db_clone, chunk, "rabbit@sunnyside")
                                    .await
                                    .unwrap();
                            }
                        });

                        let chunks: Vec<_> = entries
                            .chunks(PIPELINE_CHUNK_SIZE)
                            .map(|c| c.to_vec())
                            .collect();

                        for mut chunk in chunks {
                            chunk.par_iter_mut().for_each(annotate_entry);
                            chunk.sort_by_key(|e| e.sequence_id);
                            tx.send(chunk).await.unwrap();
                        }

                        drop(tx);
                        insert_task.await.unwrap();

                        post_insertion_operations(&db).await.unwrap();
                    });
            },
        );
    }

    group.finish();
}

fn bench_pipelined_fast_import(c: &mut Criterion) {
    let mut group = c.benchmark_group("pipelined_fast_import");
    group.sample_size(10);

    for &lines in &[250_000, 500_000] {
        let fixture_path = get_fixture_path(lines);

        if !fixture_path.exists() {
            eprintln!("Warning: Fixture not found: {:?}", fixture_path);
            continue;
        }

        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{}K", lines / 1000)),
            &fixture_path,
            |b, path| {
                b.to_async(tokio::runtime::Runtime::new().unwrap())
                    .iter(|| async {
                        let temp_dir = TempDir::new().unwrap();
                        let db_path = temp_dir.path().join("benchmark.db");
                        let db = create_database_for_bulk_import(&db_path).await.unwrap();

                        let file = File::open(path).unwrap();
                        let reader = BufReader::new(file);
                        let entries = parse_log_file(reader).unwrap().entries;

                        let (tx, mut rx) = mpsc::channel::<Vec<ParsedLogEntry>>(4);

                        let db_clone = db.clone();
                        let insert_task = tokio::spawn(async move {
                            while let Some(chunk) = rx.recv().await {
                                insert_chunk(&db_clone, chunk, "rabbit@sunnyside")
                                    .await
                                    .unwrap();
                            }
                        });

                        let chunks: Vec<_> = entries
                            .chunks(PIPELINE_CHUNK_SIZE)
                            .map(|c| c.to_vec())
                            .collect();

                        for mut chunk in chunks {
                            chunk.par_iter_mut().for_each(annotate_entry);
                            chunk.sort_by_key(|e| e.sequence_id);
                            tx.send(chunk).await.unwrap();
                        }

                        drop(tx);
                        insert_task.await.unwrap();

                        post_insertion_operations(&db).await.unwrap();
                        finalize_bulk_import(&db).await.unwrap();
                    });
            },
        );
    }

    group.finish();
}

criterion_group!(
    benches,
    bench_parse_only,
    bench_parse_and_annotate,
    bench_full_pipeline,
    bench_pipelined_full,
    bench_pipelined_fast_import
);
criterion_main!(benches);
