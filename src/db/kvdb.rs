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

use serde_json::Value;
use sqlx::MySqlPool;
use crate::chat::{create_db_tables, fill_db_tables, UserSettingKV};
use crate::db::{create_db, ConnectionConfig};

/// Key-value database Manager.
#[derive(Debug, Default)]
pub struct KeyValueDBManager {
    /// Manager MySQL connection pool.
    pool: Option<MySqlPool>,
    /// Connection config associated with DB manager.
    config: ConnectionConfig,
}

impl KeyValueDBManager {
    /// Construct new KeyValueDBManager object.
    ///
    /// # Returns
    /// - New `KeyValueDBManager` object.
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
                CREATE  PROCEDURE IF NOT EXISTS AddUserSetting(
                    IN p_user_id BIGINT,
                    IN p_setting_name VARCHAR(255),
                    IN p_setting_value VARCHAR(255)
                )
                BEGIN
                    DECLARE existing_count INT;

                    SELECT COUNT(*)
                    INTO existing_count
                    FROM User_Settings_KV
                    WHERE user_id = p_user_id AND setting_name = p_setting_name;

                    IF existing_count > 0 THEN
                        UPDATE User_Settings_KV
                        SET setting_value = p_setting_value
                        WHERE user_id = p_user_id AND setting_name = p_setting_name;
                    ELSE
                        INSERT INTO User_Settings_KV (user_id, setting_name, setting_value)
                        VALUES (p_user_id, p_setting_name, p_setting_value);
                    END IF;
                END;
                "#;

            sqlx::raw_sql(query).execute(pool).await?;

            let query =
                r#"
                CREATE PROCEDURE IF NOT EXISTS GetUserSetting(
                    IN p_user_id BIGINT,
                    IN p_setting_name VARCHAR(255),
                    OUT p_setting_value VARCHAR(255)
                )
                BEGIN
                    SELECT setting_value
                    INTO p_setting_value
                    FROM User_Settings_KV
                    WHERE user_id = p_user_id AND setting_name = p_setting_name;
                END;
                "#;

            sqlx::raw_sql(query).execute(pool).await?;

            let query =
                r#"
                CREATE FUNCTION IF NOT EXISTS GetAllUserSettings(
                    p_user_id BIGINT
                ) RETURNS JSON
                READS SQL DATA
                BEGIN
                    DECLARE settings JSON;

                    SELECT JSON_OBJECTAGG(setting_name, setting_value)
                    INTO settings
                    FROM User_Settings_KV
                    WHERE user_id = p_user_id;

                    RETURN settings;
                END;
                "#;

            sqlx::raw_sql(query).execute(pool).await?;

            Ok(())
        }
        else {
            // Handle case when connection pool was not initialized.
            eprintln!(
                "Error: connection pool is None: {}",
                "call KeyValueDBManager::connect() method first!"
            );
            Err(sqlx::Error::PoolClosed)
        }
    }

    pub async fn add_user_setting(&self, user_setting_kv: &UserSettingKV)
        -> Result<(), sqlx::Error>
    {
        if let Some(pool) = &self.pool {
            let query = "CALL AddUserSetting(?, ?, ?);";

            sqlx::query(query)
                .bind(&user_setting_kv.user_id)
                .bind(&user_setting_kv.settings_name)
                .bind(&user_setting_kv.settings_value)
                .execute(pool)
                .await?;
        }

        Ok(())
    }

    pub async fn get_user_setting(&self, user_id: i64, setting_name: String)
        -> Result<String, sqlx::Error>
    {
        if let Some(pool) = &self.pool {
            let query = "CALL GetUserSetting(?, ?, @setting_value);";
            sqlx::query(query)
                .bind(user_id)
                .bind(&setting_name)
                .execute(pool)
                .await?;

            // Fetch the value of @setting_value.
            let query = "SELECT @setting_value;";
            let row: (Option<String>,) = sqlx::query_as(query)
                .fetch_one(pool)
                .await?;

            return if let Some(value) = row.0 {
                Ok(value)
            } else {
                Err(sqlx::Error::RowNotFound)
            }
        }

        Err(sqlx::Error::RowNotFound)
    }

    pub async fn get_all_user_settings(&self, user_id: i64)
        -> Result<Option<Value>, sqlx::Error>
    {
        if let Some(pool) = &self.pool {
            let query = "SELECT GetAllUserSettings(?) AS settings;";
            let row: (Option<Value>,) = sqlx::query_as(query)
                .bind(user_id)
                .fetch_one(pool)
                .await?;

            Ok(row.0)
        } else {
            Err(sqlx::Error::RowNotFound)
        }
    }

}
