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

use sea_orm::ActiveValue;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

pub type FileMetadata = Entity;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "file_metadata")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub file_path: String,

    #[sea_orm(column_type = "JsonBinary")]
    pub rabbitmq_versions: Json,

    #[sea_orm(column_type = "JsonBinary")]
    pub erlang_versions: Json,

    pub tls_library: Option<String>,

    pub oldest_entry_at: Option<DateTimeUtc>,

    pub most_recent_entry_at: Option<DateTimeUtc>,

    pub total_lines: i64,

    pub total_entries: i64,

    #[sea_orm(column_type = "JsonBinary")]
    pub nodes: Json,

    #[sea_orm(column_type = "JsonBinary")]
    pub subsystems: Json,

    #[sea_orm(column_type = "JsonBinary")]
    pub labels: Json,

    #[sea_orm(column_type = "JsonBinary")]
    pub enabled_plugins: Json,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

impl Entity {
    pub async fn find_all(db: &DatabaseConnection) -> Result<Vec<Model>, DbErr> {
        Self::find().all(db).await
    }

    pub async fn insert_metadata(db: &DatabaseConnection, metadata: Model) -> Result<(), DbErr> {
        let active_model = ActiveModel {
            file_path: ActiveValue::Set(metadata.file_path),
            rabbitmq_versions: ActiveValue::Set(metadata.rabbitmq_versions),
            erlang_versions: ActiveValue::Set(metadata.erlang_versions),
            tls_library: ActiveValue::Set(metadata.tls_library),
            oldest_entry_at: ActiveValue::Set(metadata.oldest_entry_at),
            most_recent_entry_at: ActiveValue::Set(metadata.most_recent_entry_at),
            total_lines: ActiveValue::Set(metadata.total_lines),
            total_entries: ActiveValue::Set(metadata.total_entries),
            nodes: ActiveValue::Set(metadata.nodes),
            subsystems: ActiveValue::Set(metadata.subsystems),
            labels: ActiveValue::Set(metadata.labels),
            enabled_plugins: ActiveValue::Set(metadata.enabled_plugins),
        };

        active_model.insert(db).await?;
        Ok(())
    }
}
