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

//! Message related structs.

use crate::db::{create_table, CrudOps};
use chrono::NaiveDateTime;
use sqlx::MySqlPool;
use rand::Rng;

/// Message table.
#[derive(Debug, Default)]
pub struct Message {
    /// Message identifier.
    message_id: i64,
    /// Channel identifier where the message was sent.
    channel_id: i64,
    /// User identifier of the sender.
    user_id: i64,
    /// The text of the message.
    message_text: String,
    /// Timestamp of when the message was sent.
    timestamp: Option<NaiveDateTime>,
}

impl CrudOps for Message {
    async fn create(pool: &MySqlPool) -> Result<(), sqlx::Error> {
        let name = "Message".to_string();
        let content = String::from(
            r#"
            message_id BIGINT AUTO_INCREMENT UNIQUE,
            channel_id BIGINT,
            user_id BIGINT,
            message_text TEXT,
            timestamp DATETIME,
            PRIMARY KEY(message_id)
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
        self.channel_id = rng.gen_range(1..10000);
        self.user_id = rng.gen_range(1..10000);
        self.message_text = format!("This is a random message {}", rng.gen_range(1..10000));
        self.timestamp = Some(NaiveDateTime::from_timestamp(chrono::Utc::now().timestamp(), 0));

        // Insert the new message into the database.
        sqlx::query(
            r#"
            INSERT INTO Message
            (channel_id, user_id, message_text, timestamp)
            VALUES (?, ?, ?, ?)
            "#,
        )
            .bind(self.channel_id)
            .bind(self.user_id)
            .bind(&self.message_text)
            .bind(self.timestamp.unwrap().to_string())
            .execute(pool)
            .await?;

        Ok(())
    }
}

/// Reaction table.
#[derive(Debug, Default)]
pub struct Reaction {
    /// Reaction identifier.
    reaction_id: i64,
    /// Message identifier associated with the reaction.
    message_id: i64,
    /// User identifier who made the reaction.
    user_id: i64,
    /// Timestamp of when the reaction was made.
    timestamp: Option<NaiveDateTime>,
    /// Type of the reaction (e.g., "like", "dislike").
    reaction_type: String,
}

impl CrudOps for Reaction {
    async fn create(pool: &MySqlPool) -> Result<(), sqlx::Error> {
        let name = "Reactions".to_string();
        let content = String::from(
            r#"
            reaction_id BIGINT AUTO_INCREMENT UNIQUE,
            message_id BIGINT,
            user_id BIGINT,
            timestamp DATETIME,
            reaction_type VARCHAR(255),
            PRIMARY KEY(reaction_id)
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
        self.message_id = rng.gen_range(1..10000);
        self.user_id = rng.gen_range(1..10000);
        self.timestamp = Some(NaiveDateTime::from_timestamp(chrono::Utc::now().timestamp(), 0));
        self.reaction_type = format!("reaction_type_{}", rng.gen_range(1..5)); // Random reaction type.

        // Insert the new reaction into the database.
        sqlx::query(
            r#"
            INSERT INTO Reactions
            (message_id, user_id, timestamp, reaction_type)
            VALUES (?, ?, ?, ?)
            "#,
        )
            .bind(self.message_id)
            .bind(self.user_id)
            .bind(self.timestamp.unwrap().to_string())
            .bind(&self.reaction_type)
            .execute(pool)
            .await?;

        Ok(())
    }
}
