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

//! Channel related structs.

use crate::db::{create_table, CrudOps};
use chrono::NaiveDate;
use sqlx::MySqlPool;
use rand::Rng;

/// Channel table.
#[derive(Debug, Default)]
pub struct Channel {
    /// Channel identifier.
    channel_id: i64,
    /// Name of the channel.
    channel_name: String,
    /// Topic identifier.
    topic: i32,
    /// User identifier of the creator.
    created_by: i64,
    /// Channel creation date.
    creator: Option<NaiveDate>,
    /// Is the channel private?
    is_private: bool,
}

impl CrudOps for Channel {
    async fn create(pool: &MySqlPool) -> Result<(), sqlx::Error> {
        let name = "Channel".to_string();
        let content = String::from(
            r#"
            channel_id BIGINT AUTO_INCREMENT UNIQUE,
            channel_name TINYTEXT,
            topic INTEGER,
            created_by BIGINT,
            creator DATE,
            is_private BOOLEAN,
            PRIMARY KEY(channel_id)
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
        self.channel_name = format!("channel_{}", rng.gen_range(1..10000));
        self.topic = rng.gen_range(1..100);
        self.created_by = rng.gen_range(1..10000);
        self.creator = Some(NaiveDate::from_ymd_opt(2025, 1, 1).unwrap());
        self.is_private = rng.gen_bool(0.5); // Randomly set to true or false.

        // Insert the new channel into the database.
        sqlx::query(
            r#"
            INSERT INTO Channel
            (channel_name, topic, created_by, creator, is_private)
            VALUES (?, ?, ?, ?, ?)
            "#,
        )
            .bind(&self.channel_name)
            .bind(self.topic)
            .bind(self.created_by)
            .bind(self.creator.unwrap().to_string())
            .bind(self.is_private)
            .execute(pool)
            .await?;

        Ok(())
    }
}
