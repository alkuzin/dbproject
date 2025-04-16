// DBProject - non-relational databases tasks.
// Copyright (C) 2025 Alexander (@alkuzin).
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

//! Log related structs.

use crate::db::{create_table, CrudOps};
use chrono::NaiveDateTime;
use sqlx::MySqlPool;
use rand::Rng;

/// Log table.
#[derive(Debug, Default)]
pub struct Log {
    /// Log identifier.
    log_id: i64,
    /// Type of the event logged.
    event_type: String,
    /// User identifier associated with the event.
    user_id: i64,
    /// Channel identifier associated with the event.
    channel_id: i64,
    /// Timestamp of when the event occurred.
    timestamp: Option<NaiveDateTime>,
    /// Additional details about the event.
    details: String,
}

impl CrudOps for Log {
    async fn create(pool: &MySqlPool) -> Result<(), sqlx::Error> {
        let name = "Logs".to_string();
        let content = String::from(
            r#"
            log_id BIGINT AUTO_INCREMENT UNIQUE,
            event_type TEXT,
            user_id BIGINT,
            channel_id BIGINT,
            timestamp DATETIME,
            details TEXT,
            PRIMARY KEY(log_id)
            "#
        );

        create_table(pool, &name, &content.to_string()).await?;

        println!("Created table: {name}");
        Ok(())
    }

    async fn update(&self, _pool: &MySqlPool) -> Result<(), sqlx::Error> {
        todo!()
    }

    async fn delete(&self, _pool: &MySqlPool) -> Result<(), sqlx::Error> {
        todo!()
    }

    async fn fill_random(&mut self, pool: &MySqlPool)
                         -> Result<(), sqlx::Error>
    {
        let mut rng = rand::thread_rng();

        // Generate random values.
        self.event_type = format!("event_type_{}", rng.gen_range(1..100));
        self.user_id = rng.gen_range(1..10000);
        self.channel_id = rng.gen_range(1..10000);
        self.timestamp = Some(NaiveDateTime::from_timestamp(chrono::Utc::now().timestamp(), 0));
        self.details = format!("Details for event {}", rng.gen_range(1..10000)); // Random details.

        // Insert the new log into the database.
        sqlx::query(
            r#"
            INSERT INTO Logs
            (event_type, user_id, channel_id, timestamp, details)
            VALUES (?, ?, ?, ?, ?)
            "#,
        )
            .bind(&self.event_type)
            .bind(self.user_id)
            .bind(self.channel_id)
            .bind(self.timestamp.unwrap().to_string())
            .bind(&self.details)
            .execute(pool)
            .await?;

        Ok(())
    }
}
