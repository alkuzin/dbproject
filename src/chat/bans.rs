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

//! Ban related structs.

use crate::db::{create_table, CrudOps};
use chrono::NaiveDate;
use sqlx::MySqlPool;
use rand::Rng;

/// Ban table.
#[derive(Debug, Default)]
pub struct Ban {
    /// Ban identifier.
    ban_id: i64,
    /// Channel identifier where the ban occurred.
    channel_id: i64,
    /// User identifier of the banned user.
    user_id: i64,
    /// Date when the user was banned.
    banned_at: Option<NaiveDate>,
    /// Reason for the ban.
    reason: String,
}

impl CrudOps for Ban {
    async fn create(pool: &MySqlPool) -> Result<(), sqlx::Error> {
        let name = "Bans".to_string();
        let content = String::from(
            r#"
            ban_id INTEGER AUTO_INCREMENT UNIQUE,
            channel_id BIGINT,
            user_id BIGINT,
            banned_at DATE,
            reason TEXT,
            PRIMARY KEY(ban_id)
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
        self.banned_at = Some(NaiveDate::from_ymd_opt(2025, 1, 1).unwrap());
        self.reason = format!("Reason for ban {}", rng.gen_range(1..100)); // Random reason assignment.

        // Insert the new ban into the database.
        sqlx::query(
            r#"
            INSERT INTO Bans
            (channel_id, user_id, banned_at, reason)
            VALUES (?, ?, ?, ?)
            "#,
        )
            .bind(self.channel_id)
            .bind(self.user_id)
            .bind(self.banned_at.unwrap().to_string())
            .bind(&self.reason)
            .execute(pool)
            .await?;

        Ok(())
    }
}
