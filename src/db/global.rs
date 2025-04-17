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

//! Global database manager related declarations.

use super::{ConnectionConfig, area::{Area, AreaDB}, dump_db, restore_db, create_db};
use std::collections::HashMap;
use chrono::NaiveDate;
use sqlx::MySqlPool;
use crate::chat::User;

/// Global database manager.
#[derive(Debug, Default)]
pub struct GlobalDB {
    /// Area-specific database managers table.
    table: HashMap<Area, AreaDB>,
    /// Global MySQL connection pool.
    global_pool: Option<MySqlPool>,
    /// Core database MySQL connection pool.
    core_pool: Option<MySqlPool>,
}

impl GlobalDB {
    /// Construct new GlobalDB object.
    ///
    /// # Returns
    /// - New `GlobalDB` object.
    pub fn new() -> Self {
        Self::default()
    }

    /// Connect global database.
    ///
    /// # Parameters
    /// - `config`  - given MySQL connection config.
    ///
    /// # Returns
    /// - `Ok` - in case of success.
    /// - `sqlx::Error` - otherwise.
    pub async fn connect(&mut self, config: &ConnectionConfig)
        -> Result<(), sqlx::Error>
    {
        let global_url   = &config.url_db();
        self.global_pool = Some(MySqlPool::connect(global_url.as_str()).await?);

        if let Some(pool) = &self.global_pool {
            create_db(pool, &"CoreDB".to_string()).await?;
        }

        let core_db_url = format!("{}CoreDB", global_url);
        self.core_pool  = Some(MySqlPool::connect(&core_db_url).await?);
        self.set_procedures().await?;
        Ok(())
    }

    pub async fn set_procedures(&self) -> Result<(), sqlx::Error> {
        if let Some(pool) = &self.core_pool {
            // Procedure to add a new user.
            let query =
                r#"
                CREATE PROCEDURE IF NOT EXISTS AddUser(
                    IN p_username VARCHAR(255),
                    IN p_password_hash VARCHAR(255),
                    IN p_email VARCHAR(255),
                    IN p_region VARCHAR(255)
                )
                BEGIN
                    IF p_region = 'Russia' THEN
                        INSERT INTO AreaDB_Russia.User
                        (username, password_hash, email, created_at, last_login)
                        VALUES (p_username, p_password_hash, p_email, CURDATE(), CURDATE());
                    ELSEIF p_region = 'USA' THEN
                        INSERT INTO AreaDB_USA.User
                        (username, password_hash, email, created_at, last_login)
                        VALUES (p_username, p_password_hash, p_email, CURDATE(), CURDATE());
                    ELSE
                        SIGNAL SQLSTATE '45000' SET MESSAGE_TEXT = 'Invalid region specified';
                    END IF;
                END;
                "#;

            sqlx::raw_sql(query).execute(pool).await?;

            // Procedure to update user info.
            let query =
                r#"
                CREATE PROCEDURE IF NOT EXISTS UpdateUser(
                    IN p_user_id BIGINT,
                    IN p_username VARCHAR(255),
                    IN p_password_hash VARCHAR(255),
                    IN p_email VARCHAR(255),
                    IN p_region VARCHAR(255)
                )
                BEGIN
                    IF p_region = 'Russia' THEN
                        UPDATE AreaDB_Russia.User
                        SET username = p_username,
                            password_hash = p_password_hash,
                            email = p_email,
                            last_login = CURDATE()
                        WHERE user_id = p_user_id;
                    ELSEIF p_region = 'USA' THEN
                        UPDATE AreaDB_USA.User
                        SET username = p_username,
                            password_hash = p_password_hash,
                            email = p_email,
                            last_login = CURDATE()
                        WHERE user_id = p_user_id;
                    ELSE
                        SIGNAL SQLSTATE '45000' SET MESSAGE_TEXT = 'Invalid region specified';
                    END IF;
                END;
                "#;

            sqlx::raw_sql(query).execute(pool).await?;

            // Procedure to delete user info.
            let query =
                r#"
                CREATE PROCEDURE IF NOT EXISTS DeleteUser(
                    IN p_user_id BIGINT,
                    IN p_region VARCHAR(255)
                )
                BEGIN
                    IF p_region = 'Russia' THEN
                        DELETE FROM AreaDB_Russia.User
                        WHERE user_id = p_user_id;
                    ELSEIF p_region = 'USA' THEN
                        DELETE FROM AreaDB_USA.User
                        WHERE user_id = p_user_id;
                    ELSE
                        SIGNAL SQLSTATE '45000' SET MESSAGE_TEXT = 'Invalid region specified';
                    END IF;
                END;
                "#;

            sqlx::raw_sql(query).execute(pool).await?;
        }
        else {
            // Handle case when connection pool was not initialized.
            eprintln!(
                "Error: connection pool is None: {}",
                "call GlobalDB::connect() method first!"
            );
            return Err(sqlx::Error::PoolClosed)
        }

        Ok(())
    }

    /// Insert area-specific database manager.
    ///
    /// # Parameters
    /// - `area_db` - given database manager.
    ///
    /// # Returns
    /// - `Ok` - in case of success.
    /// - `sqlx::Error` - otherwise.
    pub async fn insert(&mut self, area_db: AreaDB)
        -> Result<(), sqlx::Error>
    {
        let config = &area_db.config().clone();

        if let Some(pool) = &self.global_pool {
            create_db(pool, &config.database).await?;
        }

        let mut manager = area_db;
        let url = config.url_db();
        manager.connect(url.as_str()).await?;

        self.table.insert(manager.area(), manager);
        Ok(())
    }

    /// Dump database by specific area.
    ///
    /// # Parameters
    /// - `area` - given manager area.
    ///
    /// # Returns
    /// - `Ok` - in case of success.
    /// - `sqlx::Error` - otherwise.
    pub async fn dump_db_by_area(&self, area: &Area)
        -> Result<(), sqlx::Error>
    {
        // Check if area is correct.
        if !self.table.contains_key(area) {
            return Err(sqlx::Error::RowNotFound);
        }

        let area_db   = self.table.get(area).unwrap();
        let dump_name = format!("{}_backup.sql", area_db.name());

        dump_db(&area_db.config(), &dump_name)?;

        // Store backup on current server.
        let mut config  = area_db.config().clone();
        config.database = format!("{}_BACKUP", config.database);

        if let Some(pool) = &self.global_pool {
            create_db(pool, &config.database).await?;
        }

        restore_db(&config, &dump_name)?;

        Ok(())
    }

    /// Restore database by specific area.
    ///
    /// # Parameters
    /// - `area` - given manager area.
    ///
    /// # Returns
    /// - `Ok` - in case of success.
    /// - `sqlx::Error` - otherwise.
    pub async fn restore_db_by_area(&self, area: &Area) -> Result<(), sqlx::Error> {
        // Check if area is correct.
        if !self.table.contains_key(area) {
            return Err(sqlx::Error::RowNotFound);
        }

        let area_db   = self.table.get(area).unwrap();
        let dump_name = format!("{}_backup.sql", area_db.name());

        restore_db(&area_db.config(), &dump_name)?;
        Ok(())
    }

    /// Add a new user to the database based on the specified region.
    ///
    /// # Parameters
    /// - `user` - given user info.
    /// - `area` - given manager area.
    ///
    /// # Returns
    /// - `Ok` - in case of success.
    /// - `sqlx::Error` - otherwise.
    pub async fn add_user(&self, user: &User, area: &Area) -> Result<(), sqlx::Error> {
        let query = "CALL AddUser(?, ?, ?, ?);";

        if let Some(pool) = &self.core_pool {
            sqlx::query(query)
                .bind(&user.username)
                .bind(&user.password_hash)
                .bind(&user.email)
                .bind(&area.to_string())
                .execute(pool)
                .await?;
        }

        Ok(())
    }

    /// Update the information of an existing user in the database.
    ///
    /// # Parameters
    /// - `user` - given user info.
    /// - `area` - given manager area.
    ///
    /// # Returns
    /// - `Ok` - in case of success.
    /// - `sqlx::Error` - otherwise.
    pub async fn update_user(&self, user: &User, area: &Area) -> Result<(), sqlx::Error> {
        let query = "CALL UpdateUser(?, ?, ?, ?, ?);";

        if let Some(pool) = &self.core_pool {
            sqlx::query(query)
                .bind(&user.user_id)
                .bind(&user.username)
                .bind(&user.password_hash)
                .bind(&user.email)
                .bind(&area.to_string())
                .execute(pool)
                .await?;
        }

        Ok(())
    }

    /// Delete a user from the database based on the specified region.
    ///
    /// # Parameters
    /// - `user` - given user info.
    /// - `area` - given manager area.
    ///
    /// # Returns
    /// - `Ok` - in case of success.
    /// - `sqlx::Error` - otherwise.
    pub async fn delete_user(&self, user_id: i64, area: &Area) -> Result<(), sqlx::Error> {
        let query = "CALL DeleteUser(?, ?);";

        if let Some(pool) = &self.core_pool {
            sqlx::query(query)
                .bind(&user_id)
                .bind(&area.to_string())
                .execute(pool)
                .await?;
        }

        Ok(())
    }

    /// Print all users from the specified region.
    ///
    /// # Parameters
    /// - `pool` - The database connection pool.
    /// - `region` - The region to filter users by (e.g., "Russia" or "USA").
    ///
    /// # Returns
    /// - `Result<(), sqlx::Error>` - Ok if successful, or an error if something goes wrong.
    pub async fn print_users_by_area(&self, area: &Area) -> Result<(), sqlx::Error> {
        let area_db = self.table.get(area).unwrap();
        let query   = format!("SELECT * FROM {}.User;", area_db.config().database);

        if let Some(pool) = &self.global_pool {
            let users: Vec<User> = sqlx::query_as(query.as_str())
                .fetch_all(pool)
                .await?;

            for user in users {
                println!(
                    "User ID: {}, Username: {}, Email: {}, Created At: {:?}, Last Login: {:?}",
                    user.user_id, user.username, user.email, user.created_at, user.last_login
                );
            }
        }

        Ok(())
    }

    pub async fn test_procedures(&self) -> Result<(), sqlx::Error> {
        // Create a test user
        let test_user = User {
            user_id:        2,
            username:       "test_user".to_string(),
            password_hash:  "hashed_password".to_string(),
            email:          "test_user@example.com".to_string(),
            created_at:     None,
            last_login:     None,
        };

        let area    = Area::Usa;
        let area_db = self.table.get(&area).unwrap();

        println!("Test procedures");
        println!("{:#?}", test_user);

        println!("Before test:");
        self.print_users_by_area(&area).await?;

        println!("Test AddUser():");
        // Add the user
        self.add_user(&test_user, &area).await?;

        println!("After test AddUser():");
        self.print_users_by_area(&area).await?;

        // Update the user
        let updated_user = User {
            user_id:       test_user.user_id,
            username:      "updated_user".to_string(),
            password_hash: "new_hashed_password".to_string(),
            email:         "updated_user@example.com".to_string(),
            created_at:    test_user.created_at,
            last_login:    test_user.last_login,
        };

        self.update_user(&updated_user, &area).await?;

        println!("After test UpdateUser():");
        self.print_users_by_area(&area).await?;

        // Delete the user
        self.delete_user(updated_user.user_id, &area).await?;

        println!("After test DeleteUser():");
        self.print_users_by_area(&area).await?;

        if let Some(pool) = &self.global_pool {
            let query = format!("TRUNCATE TABLE {}.User;", area_db.config().database);

            sqlx::query(query.as_str())
                .bind(&area_db.config().database)
                .execute(pool)
                .await?;
        }

        Ok(())
    }

    /// Get user count by each region.
    ///
    /// # Returns
    /// - Vector of tuple of area name & its user count - in case of success.
    /// - `sqlx::Error` - otherwise.
    pub async fn get_user_count(&self) -> Result<Vec<(String, i64)>, sqlx::Error> {
        let query =
            r#"
            SELECT
                'Russia' AS region,
                COUNT(*) AS user_count
            FROM
                AreaDB_Russia.User
            UNION ALL
            SELECT
                'USA' AS region,
                COUNT(*) AS user_count
            FROM
                AreaDB_USA.User;
            "#;

        if let Some(pool) = &self.core_pool {
            let count = sqlx::query_as::<_, (String, i64)>(query)
                .fetch_all(pool)
                .await?;

            return Ok(count);
        }

        Err(sqlx::Error::RowNotFound)
    }

    /// Get message count by each region.
    ///
    /// # Returns
    /// - Vector of tuple of area name & its message count - in case of success.
    /// - `sqlx::Error` - otherwise.
    pub async fn get_message_count(&self) -> Result<Vec<(String, i64)>, sqlx::Error> {
        let query =
            r#"
            WITH MessageCounts AS (
                SELECT
                    'Russia' AS region,
                    COUNT(m.message_id) AS message_count
                FROM
                    AreaDB_Russia.Message m
                UNION ALL
                SELECT
                    'USA' AS region,
                    COUNT(m.message_id) AS message_count
                FROM
                    AreaDB_USA.Message m
            )

            SELECT
                region,
                CAST(SUM(message_count) AS SIGNED) AS total_messages
            FROM
                MessageCounts
            GROUP BY
                region;
            "#;

        if let Some(pool) = &self.core_pool {
            let count = sqlx::query_as::<_, (String, i64)>(query)
                .fetch_all(pool)
                .await?;

            return Ok(count);
        }

        Err(sqlx::Error::RowNotFound)
    }

    /// Get the top 5 channels by message count for each region.
    ///
    /// # Returns
    /// - Vector of tuples of area, channel, and its message count - in case of success.
    /// - `sqlx::Error` - otherwise.
    pub async fn get_channels_with_no_messages(&self) -> Result<Vec<(String, String)>, sqlx::Error> {
        let query = r#"
            SELECT
                c.channel_name,
                'Russia' AS region
            FROM
                AreaDB_Russia.Channel c
            LEFT JOIN
                AreaDB_Russia.Message m ON c.channel_id = m.channel_id
            WHERE
                m.message_id IS NULL

            UNION ALL

            SELECT
                c.channel_name,
                'USA' AS region
            FROM
                AreaDB_USA.Channel c
            LEFT JOIN
                AreaDB_USA.Message m ON c.channel_id = m.channel_id
            WHERE
                m.message_id IS NULL;
        "#;

        if let Some(pool) = &self.core_pool {
            let channels = sqlx::query_as::<_, (String, String)>(query)
                .fetch_all(pool)
                .await?;

            return Ok(channels);
        }

        Err(sqlx::Error::RowNotFound)
    }

    pub async fn get_last_activity(&self)
        -> Result<Vec<(String, String, Option<NaiveDate>)>, sqlx::Error>
    {
        let query = r#"
            SELECT
                'Russia' AS region,
                u.username,
                MAX(l.timestamp) AS last_activity
            FROM
                AreaDB_Russia.User u
            LEFT JOIN
                AreaDB_Russia.Logs l ON u.user_id = l.user_id
            GROUP BY
                u.user_id

            UNION ALL

            SELECT
                'USA' AS region,
                u.username,
                MAX(l.timestamp) AS last_activity
            FROM
                AreaDB_USA.User u
            LEFT JOIN
                AreaDB_USA.Logs l ON u.user_id = l.user_id
            GROUP BY
                u.user_id;
        "#;

        if let Some(pool) = &self.core_pool {
            let activities = sqlx::query_as::<_, (String, String, Option<NaiveDate>)>(query)
                .fetch_all(pool)
                .await?;

            return Ok(activities);
        }

        Err(sqlx::Error::RowNotFound)
    }

    pub async fn get_total_reactions(&self) -> Result<Vec<(i64, i64)>, sqlx::Error> {
        let query = r#"
            SELECT
                user_id,
                CAST(SUM(reaction_count) AS SIGNED) AS total_reactions
            FROM (
                SELECT
                    r.user_id,
                    COUNT(r.reaction_id) AS reaction_count
                FROM
                    AreaDB_Russia.Reactions r
                GROUP BY
                    r.user_id
                UNION ALL
                SELECT
                    r.user_id,
                    COUNT(r.reaction_id) AS reaction_count
                FROM
                    AreaDB_USA.Reactions r
                GROUP BY
                    r.user_id
            ) AS combined_reactions
            GROUP BY
                user_id;
        "#;

        if let Some(pool) = &self.core_pool {
            let reactions = sqlx::query_as::<_, (i64, i64)>(query)
                .fetch_all(pool)
                .await?;

            return Ok(reactions);
        }

        Err(sqlx::Error::RowNotFound)
    }

    pub async fn test_requests(&self) -> Result<(), sqlx::Error> {
        println!("Test get_user_count():");
        let user_count = self.get_user_count().await?;

        for (area, count) in user_count {
            println!("Area: {area}, Count: {count}");
        }

        println!("Test get_message_count():");
        let msg_count = self.get_message_count().await?;

        for (area, count) in msg_count {
            println!("Area: {area}, Count: {count}");
        }

        println!("Test get_channels_with_no_messages():");
        let channels = self.get_channels_with_no_messages().await?;

        for (channel_name, region) in channels {
            println!("Channel: {}, Region: {}", channel_name, region);
        }

        println!("Test get_last_activity():");
        let activities = self.get_last_activity().await?;

        for (region, username, last_activity) in activities {
            match last_activity {
                Some(timestamp) => println!("Region: {}, Username: {}, Last Activity: {}", region, username, timestamp),
                None => println!("Region: {}, Username: {}, Last Activity: None", region, username),
            }
        }

        println!("Test get_total_reactions():");
        let reactions = self.get_total_reactions().await?;

        for (user_id, total_reactions) in reactions {
            println!("User ID: {}, Total Reactions: {}", user_id, total_reactions);
        }

        Ok(())
    }
}