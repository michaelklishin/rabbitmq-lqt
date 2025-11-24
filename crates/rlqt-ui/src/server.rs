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
use axum::Router;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use clap::ArgMatches;
use rlqt_lib::open_database;
use sea_orm::DatabaseConnection;
use std::path::PathBuf;
use std::sync::Arc;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<DatabaseConnection>,
}

pub async fn handle_serve_command(args: &ArgMatches) -> i32 {
    let db_path: PathBuf = args
        .get_one::<String>("input_db_file_path")
        .expect("input_db_file_path is required")
        .into();

    let host = args
        .get_one::<String>("host")
        .expect("host has a default value");

    let port = args
        .get_one::<String>("port")
        .expect("port has a default value");

    if !db_path.exists() {
        log::error!("Database file does not exist: {}", db_path.display());
        eprintln!("Database file does not exist: {}", db_path.display());
        return 1;
    }

    if !db_path.is_file() {
        log::error!("Database path is not a file: {}", db_path.display());
        eprintln!("Database path is not a file: {}", db_path.display());
        return 1;
    }

    let db = match open_database(&db_path).await {
        Ok(db) => db,
        Err(e) => {
            log::error!("Failed to open database: {}", e);
            return 1;
        }
    };

    let state = AppState { db: Arc::new(db) };

    let app = create_router(state);

    let addr = format!("{}:{}", host, port);
    let listener = match tokio::net::TcpListener::bind(&addr).await {
        Ok(listener) => listener,
        Err(e) => {
            log::error!("Failed to bind to {}: {}", addr, e);
            return 1;
        }
    };

    log::info!("Server listening on http://{}", addr);

    if let Err(e) = axum::serve(listener, app).await {
        log::error!("Server error: {}", e);
        return 1;
    }

    0
}

fn create_router(state: AppState) -> Router {
    let api_routes = Router::new()
        .route("/logs", get(logs::query_logs))
        .route("/metadata", get(metadata::get_metadata))
        .route("/stats", get(metadata::get_stats))
        .route("/file-metadata", get(metadata::get_file_metadata))
        .with_state(state.clone());

    Router::new()
        .route("/", get(root_handler))
        .nest("/api", api_routes)
        .nest_service(
            "/assets",
            tower_http::services::ServeDir::new("crates/rlqt-ui/frontend/dist/assets"),
        )
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
}

#[allow(dead_code)]
pub fn create_router_for_testing(db: Arc<DatabaseConnection>) -> Router {
    let state = AppState { db };
    let api_routes = Router::new()
        .route("/logs", get(logs::query_logs))
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
