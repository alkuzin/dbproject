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

use crate::{chat::{create_db_tables, fill_db_tables}, db::ConnectionConfig};
use sqlx::{MySqlPool, Row};
use crate::chat::UserProfile;
use crate::db::create_db;

/// Document-oriented database Manager.
#[derive(Debug, Default)]
pub struct DocDBManager {
    /// Manager MySQL connection pool.
    pool: Option<MySqlPool>,
    /// Connection config associated with DocDBManager.
    config: ConnectionConfig,
}

impl DocDBManager {
    /// Construct new DocDBManager object.
    ///
    /// # Returns
    /// - New `DocDBManager` object.
    pub fn new() -> Self {
        Self::default()
    }

    /// Connect database.
    ///
    /// # Parameters
    /// - `config` - given MySQL connection config.
    ///
    /// # Returns
    /// - `Ok` - in case of success.
    /// - `sqlx::Error` - otherwise.
    pub async fn connect(&mut self, config: ConnectionConfig)
        -> Result<(), sqlx::Error>
    {
        let pool = MySqlPool::connect(config.url().as_str()).await?;
        create_db(&pool, &config.database).await?;

        let pool = MySqlPool::connect(config.url_db().as_str()).await?;

        create_db_tables(&pool).await?;
        fill_db_tables(&pool, 1).await?;
        self.pool   = Some(pool);
        self.config = config;

        Ok(())
    }

    pub async fn set_procedures(&self) -> Result<(), sqlx::Error> {
        if let Some(pool) = &self.pool {
            let query =
                r#"
                CREATE PROCEDURE IF NOT EXISTS AddUserProfileData(
                    IN p_profile_id           BIGINT,
                    IN p_bio                  TEXT,
                    IN p_profile_picture_url  TEXT,
                    IN p_location             TEXT
                )
                BEGIN
                    DECLARE v_profile_data JSON;

                    SELECT profile_data INTO v_profile_data
                    FROM User_Profiles
                    WHERE profile_id = p_profile_id;

                    IF v_profile_data IS NULL THEN
                        SET v_profile_data = JSON_OBJECT(
                            'bio', p_bio,
                            'profile_picture_url', p_profile_picture_url,
                            'location', p_location
                        );
                    ELSE
                        SET v_profile_data = JSON_SET(v_profile_data, '$.bio', p_bio);
                        SET v_profile_data = JSON_SET(v_profile_data, '$.profile_picture_url', p_profile_picture_url);
                        SET v_profile_data = JSON_SET(v_profile_data, '$.location', p_location);
                    END IF;

                    UPDATE User_Profiles
                    SET profile_data = v_profile_data
                    WHERE profile_id = p_profile_id;
                END;
            "#;

            sqlx::raw_sql(query).execute(pool).await?;

            let query =
                r#"
                CREATE PROCEDURE IF NOT EXISTS GetUserProfileData(
                    IN  p_profile_id          BIGINT,
                    OUT p_bio                 TEXT,
                    OUT p_profile_picture_url TEXT,
                    OUT p_location            TEXT
                )
                BEGIN
                    DECLARE v_profile_data JSON;

                    SELECT profile_data INTO v_profile_data
                    FROM User_Profiles
                    WHERE profile_id = p_profile_id;

                    SET p_bio                 = JSON_UNQUOTE(JSON_EXTRACT(v_profile_data, '$.bio'));
                    SET p_profile_picture_url = JSON_UNQUOTE(JSON_EXTRACT(v_profile_data, '$.profile_picture_url'));
                    SET p_location            = JSON_UNQUOTE(JSON_EXTRACT(v_profile_data, '$.location'));
                END;
            "#;

            sqlx::raw_sql(query).execute(pool).await?;

            let query =
                r#"
                CREATE TRIGGER IF NOT EXISTS UpdateProfileDataBeforeUpdate
                BEFORE UPDATE ON User_Profiles
                FOR EACH ROW
                BEGIN
                    SET NEW.profile_data = JSON_SET(NEW.profile_data, '$.bio', NEW.bio);
                    SET NEW.profile_data = JSON_SET(NEW.profile_data, '$.profile_picture_url', NEW.profile_picture_url);
                    SET NEW.profile_data = JSON_SET(NEW.profile_data, '$.location', NEW.location);
                END;
                "#;

            sqlx::raw_sql(query).execute(pool).await?;
            Ok(())
        }
        else {
            // Handle case when connection pool was not initialized.
            eprintln!(
                "Error: connection pool is None: {}",
                "call DocDBManager::connect() method first!"
            );
            Err(sqlx::Error::PoolClosed)
        }
    }

    pub async fn add_user_profile_data(&self, user_profile: &UserProfile)
        -> Result<(), sqlx::Error>
    {
        if let Some(pool) = &self.pool {
            // Insert the new user profile into the database.
            sqlx::query(
                r#"
                INSERT INTO User_Profiles
                (user_id, bio, profile_picture_url, location)
                VALUES (?, ?, ?, ?)
                "#,
            )
                .bind(&user_profile.profile_id)
                .bind(&user_profile.bio)
                .bind(&user_profile.profile_picture_url)
                .bind(&user_profile.location)
                .execute(pool)
                .await?;

            sqlx::query("CALL AddUserProfileData(?, ?, ?, ?)")
                .bind(&user_profile.profile_id)
                .bind(&user_profile.bio)
                .bind(&user_profile.profile_picture_url)
                .bind(&user_profile.location)
                .execute(pool)
                .await?;
        }

        Ok(())
    }

    pub async fn get_user_profile_data(&self, profile_id: i64)
        -> Result<UserProfile, sqlx::Error>
    {
        let mut user_profile = UserProfile::default();

        if let Some(pool) = &self.pool {
            sqlx::query("CALL GetUserProfileData(?, @bio, @profile_picture_url, @location);")
                .bind(&profile_id)
                .execute(pool)
                .await?;

            let row: (Option<String>, Option<String>, Option<String>) =
                sqlx::query_as("SELECT @bio, @profile_picture_url, @location;")
                .fetch_one(pool)
                .await?;

            user_profile.bio                 = row.0.unwrap_or_default();
            user_profile.profile_picture_url = row.1.unwrap_or_default();
            user_profile.location            = row.2.unwrap_or_default();

            println!("Bio: {}", user_profile.bio);
            println!("Profile Picture URL: {}", user_profile.profile_picture_url);
            println!("Location: {}", user_profile.location);
            println!("CALL DIRECTLY IN DB");
        }

        Ok(user_profile)
    }

    pub async fn test_trigger(&self, user_profile: &UserProfile) -> Result<(), sqlx::Error>
    {
        if let Some(pool) = &self.pool {
            let query =
                r#"
                UPDATE User_Profiles
                SET bio = ?, profile_picture_url = ?, location = ?
                WHERE profile_id = ?;
                "#;

            sqlx::query(query)
                .bind(&user_profile.bio)
                .bind(&user_profile.profile_picture_url)
                .bind(&user_profile.location)
                .bind(&user_profile.profile_id)
                .execute(pool)
                .await?;

            let profile_data: (Option<String>, Option<String>, Option<String>) =
                sqlx::query_as("SELECT bio, profile_picture_url, location FROM User_Profiles WHERE profile_id = ?")
                .bind(&user_profile.profile_id)
                .fetch_one(pool)
                .await?;

            println!("Updated Profile Data: {:?}", profile_data);
        }

        Ok(())
    }

    pub async fn count_messages_by_channel(&self) -> Result<Vec<(String, i64)>, sqlx::Error> {
        let query =
            r#"
        SELECT JSON_UNQUOTE(JSON_EXTRACT(CAST(message_data AS CHAR), '$.channel_id')) AS channel_id, COUNT(*) AS message_count
        FROM Message
        GROUP BY JSON_UNQUOTE(JSON_EXTRACT(CAST(message_data AS CHAR), '$.channel_id'));
        "#;

        if let Some(pool) = &self.pool {
            let results = sqlx::query_as::<_, (String, i64)>(query)
                .fetch_all(pool)
                .await?;

            return Ok(results)
        }

        Err(sqlx::Error::RowNotFound)
    }

}
