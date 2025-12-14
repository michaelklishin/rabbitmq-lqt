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

use crate::api::{logs, metadata};
use crate::errors::ServerError;
use axum::Router;
use axum::extract::Path as AxumPath;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use clap::ArgMatches;
use include_dir::{Dir, include_dir};
use rlqt_lib::rel_db::FileMetadata;
use rlqt_lib::{DatabaseConnection, NodeLogEntry, open_database};
use std::io::{Error as IoError, ErrorKind};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;

static ASSETS_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/frontend/dist/assets");

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<DatabaseConnection>,
}

pub async fn handle_serve_command(args: &ArgMatches) -> Result<(), ServerError> {
    let db_path: PathBuf = args
        .get_one::<String>("input_db_file_path")
        .expect("input_db_file_path is required")
        .into();

    let host = args
        .get_one::<String>("host")
        .expect("host has a default value");

    let port: u16 = args
        .get_one::<String>("port")
        .expect("port has a default value")
        .parse()
        .expect("port must be a valid number");

    run_server(&db_path, host, port).await
}

pub async fn run_server(db_path: &Path, host: &str, port: u16) -> Result<(), ServerError> {
    if !db_path.exists() {
        return Err(ServerError::Io(IoError::new(
            ErrorKind::NotFound,
            format!("Database file does not exist: {}", db_path.display()),
        )));
    }

    if !db_path.is_file() {
        return Err(ServerError::Io(IoError::new(
            ErrorKind::InvalidInput,
            format!("Database path is not a file: {}", db_path.display()),
        )));
    }

    let db = open_database(db_path)?;

    log::info!("rabbitmq-lqt v{}", env!("CARGO_PKG_VERSION"));

    let entry_count = NodeLogEntry::count_all(&db).unwrap_or(0);
    log::info!(
        "Database at {} contains {} log entries",
        db_path.display(),
        entry_count
    );

    if let Ok(file_metadata_list) = FileMetadata::find_all(&db) {
        let oldest = file_metadata_list
            .iter()
            .filter_map(|m| m.oldest_entry_at)
            .min();
        let newest = file_metadata_list
            .iter()
            .filter_map(|m| m.most_recent_entry_at)
            .max();

        if let (Some(oldest_dt), Some(newest_dt)) = (oldest, newest) {
            log::info!(
                "Date range: {} to {}",
                oldest_dt.format("%Y-%m-%d %H:%M:%S UTC"),
                newest_dt.format("%Y-%m-%d %H:%M:%S UTC")
            );
        }
    }

    let state = AppState { db: Arc::new(db) };
    let app = create_router(state);

    let addr = format!("{}:{}", host, port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;

    let url = format!("http://{}", addr);
    log::info!("");
    log::info!("Server listening on {}", url);

    if let Err(e) = opener::open(&url) {
        log::warn!("Could not open browser: {}", e);
    }

    axum::serve(listener, app).await?;

    Ok(())
}

fn create_router(state: AppState) -> Router {
    let api_routes = Router::new()
        .route("/logs", get(logs::query_logs))
        .route("/logs/ql", get(logs::query_logs_by_ql))
        .route("/logs/preset/{preset}", get(logs::query_logs_by_preset))
        .route("/metadata", get(metadata::get_metadata))
        .route("/stats", get(metadata::get_stats))
        .route("/file-metadata", get(metadata::get_file_metadata))
        .with_state(state.clone());

    Router::new()
        .route("/", get(root_handler))
        .nest("/api", api_routes)
        .route("/assets/{*path}", get(assets_handler))
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
}

#[allow(dead_code)]
pub fn create_router_for_testing(db: Arc<DatabaseConnection>) -> Router {
    let state = AppState { db };
    let api_routes = Router::new()
        .route("/logs", get(logs::query_logs))
        .route("/logs/ql", get(logs::query_logs_by_ql))
        .route("/logs/preset/{preset}", get(logs::query_logs_by_preset))
        .route("/metadata", get(metadata::get_metadata))
        .route("/stats", get(metadata::get_stats))
        .route("/file-metadata", get(metadata::get_file_metadata))
        .with_state(state);

    Router::new().nest("/api", api_routes)
}

async fn root_handler() -> impl IntoResponse {
    (
        StatusCode::OK,
        [("content-type", "text/html")],
        include_str!("../frontend/dist/index.html"),
    )
}

async fn assets_handler(AxumPath(path): AxumPath<String>) -> impl IntoResponse {
    let content_type = if path.ends_with(".js") {
        "application/javascript"
    } else if path.ends_with(".wasm") {
        "application/wasm"
    } else if path.ends_with(".css") {
        "text/css"
    } else if path.ends_with(".woff2") {
        "font/woff2"
    } else if path.ends_with(".woff") {
        "font/woff"
    } else if path.ends_with(".svg") {
        "image/svg+xml"
    } else if path.ends_with(".png") {
        "image/png"
    } else {
        "application/octet-stream"
    };

    match ASSETS_DIR.get_file(&path) {
        Some(file) => (
            StatusCode::OK,
            [("content-type", content_type)],
            file.contents().to_vec(),
        ),
        None => (
            StatusCode::NOT_FOUND,
            [("content-type", "text/plain")],
            b"Not found".to_vec(),
        ),
    }
}
