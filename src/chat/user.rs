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

//! User related structs.

use crate::db::{create_table, CrudOps};
use chrono::NaiveDate;
use sqlx::MySqlPool;
use rand::Rng;

/// User table.
#[derive(Debug, Default)]
pub struct User {
    /// User identifier.
    user_id: i64,
    /// Name of the user.
    username: String,
    /// Password hash in string representation.
    password_hash: String,
    /// User email.
    email: String,
    /// User creation time.
    created_at: Option<NaiveDate>,
    /// User last login time.
    last_login: Option<NaiveDate>,
}

impl CrudOps for User {
    async fn create(pool: &MySqlPool) -> Result<(), sqlx::Error> {
        let name = "User".to_string();
        let content = String::from(
            r#"
            user_id BIGINT AUTO_INCREMENT UNIQUE,
            username TINYTEXT,
            password_hash LONGTEXT,
            email TEXT,
            created_at DATE,
            last_login DATE,
            PRIMARY KEY(user_id)
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
        self.username      = format!("user_{}", rng.gen_range(1..10000));
        self.password_hash = format!("hash_{}", rng.gen_range(1..10000));
        self.email         = format!("user{}@example.com", rng.gen_range(1..10000));
        self.created_at    = Some(NaiveDate::from_ymd_opt(2025, 1, 1).unwrap());
        self.last_login    = Some(NaiveDate::from_ymd_opt(2025, 1, 2).unwrap());

        // Insert the new user into the database.
        sqlx::query(
            r#"
            INSERT INTO User
            (username, password_hash, email, created_at, last_login)
            VALUES (?, ?, ?, ?, ?)
            "#,
        )
            .bind(&self.username)
            .bind(&self.password_hash)
            .bind(&self.email)
            .bind(self.created_at.unwrap().to_string())
            .bind(self.last_login.unwrap().to_string())
            .execute(pool)
            .await?;

        Ok(())
    }
}

/// ChannelUser table.
#[derive(Debug, Default)]
pub struct ChannelUser {
    /// Channel user identifier.
    channel_user_id: i64,
    /// Channel identifier.
    channel_id: i64,
    /// User identifier.
    user_id: i64,
    /// Date when the user joined the channel.
    joined_at: Option<NaiveDate>,
    /// Role of the user in the channel.
    role: String,
}

impl CrudOps for ChannelUser {
    async fn create(pool: &MySqlPool) -> Result<(), sqlx::Error> {
        let name = "Channel_Users".to_string();
        let content = String::from(
            r#"
            channel_user_id BIGINT,
            channel_id BIGINT AUTO_INCREMENT UNIQUE,
            user_id BIGINT,
            joined_at DATE,
            role TEXT,
            PRIMARY KEY(channel_user_id)
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
        self.channel_user_id = rng.gen_range(1..10000);
        self.channel_id = rng.gen_range(1..10000);
        self.user_id = rng.gen_range(1..10000);
        self.joined_at = Some(NaiveDate::from_ymd_opt(2025, 1, 1).unwrap());
        self.role = format!("role_{}", rng.gen_range(1..5)); // Random role assignment.

        // Insert the new channel user into the database.
        sqlx::query(
            r#"
            INSERT INTO Channel_Users
            (channel_user_id, channel_id, user_id, joined_at, role)
            VALUES (?, ?, ?, ?, ?)
            "#,
        )
            .bind(self.channel_user_id)
            .bind(self.channel_id)
            .bind(self.user_id)
            .bind(self.joined_at.unwrap().to_string())
            .bind(&self.role)
            .execute(pool)
            .await?;

        Ok(())
    }
}

/// UserProfile table.
#[derive(Debug, Default)]
pub struct UserProfile {
    /// Profile identifier.
    profile_id: i64,
    /// User identifier associated with the profile.
    user_id: i64,
    /// Biography of the user.
    bio: String,
    /// URL of the user's profile picture.
    profile_picture_url: String,
    /// Location of the user.
    location: String,
}

impl CrudOps for UserProfile {
    async fn create(pool: &MySqlPool) -> Result<(), sqlx::Error> {
        let name = "User_Profiles".to_string();
        let content = String::from(
            r#"
            profile_id BIGINT AUTO_INCREMENT UNIQUE,
            user_id BIGINT,
            bio TEXT,
            profile_picture_url TEXT,
            location TEXT,
            PRIMARY KEY(profile_id)
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
        self.user_id = rng.gen_range(1..10000);
        self.bio = format!("This is a random bio for user {}", rng.gen_range(1..10000));
        self.profile_picture_url = format!("https://example.com/profile_pictures/user_{}.png", rng.gen_range(1..10000));
        self.location = format!("Location {}", rng.gen_range(1..100)); // Random location.

        // Insert the new user profile into the database.
        sqlx::query(
            r#"
            INSERT INTO User_Profiles
            (user_id, bio, profile_picture_url, location)
            VALUES (?, ?, ?, ?)
            "#,
        )
            .bind(self.user_id)
            .bind(&self.bio)
            .bind(&self.profile_picture_url)
            .bind(&self.location)
            .execute(pool)
            .await?;

        Ok(())
    }
}
