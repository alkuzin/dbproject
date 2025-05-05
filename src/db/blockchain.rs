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
use crate::db::{create_db, create_table, ConnectionConfig};

/// Blockchain database Manager.
#[derive(Debug, Default)]
pub struct BlockchainDBManager {
    /// Manager MySQL connection pool.
    pool: Option<MySqlPool>,
    /// Connection config associated with DB manager.
    config: ConnectionConfig,
}

impl BlockchainDBManager {
    /// Construct new BlockchainDBManager object.
    ///
    /// # Returns
    /// - New `BlockchainDBManager` object.
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

        self.set_tables().await?;
        Ok(())
    }

    async fn set_tables(&self) -> Result<(), sqlx::Error> {
        if let Some(pool) = &self.pool {

            let content = String::from(
                r#"
                block_id BIGINT PRIMARY KEY  AUTO_INCREMENT UNIQUE,
                previous_block_hash VARCHAR(64),
                message_id BIGINT,
                user_id BIGINT,
                channel_id BIGINT,
                message_text TEXT,
                timestamp DATETIME,
                block_hash VARCHAR(64)
                "#
            );

            create_table(pool, &"Message_Block".to_string(), &content).await?;

            let content = String::from(
                r#"
                transaction_id BIGINT PRIMARY KEY  AUTO_INCREMENT UNIQUE,
                user_id BIGINT,
                action_type VARCHAR(255),
                channel_id BIGINT,
                timestamp DATETIME,
                block_hash VARCHAR(64)
                "#
            );

            create_table(pool, &"Transaction_Log".to_string(), &content).await?;
        }

        Ok(())
    }

    pub async fn set_procedures(&self) -> Result<(), sqlx::Error> {
        if let Some(pool) = &self.pool {
            let query =
                r#"
                CREATE PROCEDURE IF NOT EXISTS SendMessage(
                    IN p_user_id BIGINT,
                    IN p_channel_id BIGINT,
                    IN p_message_text TEXT
                )
                BEGIN
                    DECLARE v_block_id BIGINT;
                    DECLARE v_previous_block_hash VARCHAR(64);
                    DECLARE v_block_hash VARCHAR(64);

                    -- Получаем идентификатор последнего блока
                    SELECT COALESCE(MAX(block_id), 0) + 1
                    INTO v_block_id
                    FROM Message_Block;

                    -- Получение хэша предыдущего блока.
                    SELECT COALESCE(MAX(block_hash), '')
                    INTO v_previous_block_hash
                    FROM Message_Block;

                    -- Вычисление хэша текущего блока.
                    SET v_block_hash = SHA2(
                        CONCAT(v_previous_block_hash, p_message_text, NOW()),
                        256
                    );

                    -- Вставка сообщения в блокчейн.
                    INSERT INTO Message_Block (block_id, previous_block_hash,
                    message_id, user_id, channel_id, message_text, timestamp,
                    block_hash)
                    VALUES (v_block_id, v_previous_block_hash, NULL, p_user_id,
                    p_channel_id, p_message_text, NOW(), v_block_hash);

                    -- Сохранение транзакции.
                    INSERT INTO Transaction_Log (transaction_id, user_id,
                    action_type, channel_id, timestamp, block_hash)
                    VALUES (NULL, p_user_id, 'SEND_MESSAGE', p_channel_id,
                    NOW(), v_block_hash);
                END;
                "#;

            sqlx::raw_sql(query).execute(pool).await?;

            let query =
                r#"
                CREATE PROCEDURE IF NOT EXISTS GetUserTransactions(
                    IN p_user_id BIGINT
                )
                BEGIN
                    SELECT transaction_id, action_type, channel_id, timestamp,
                    block_hash
                    FROM Transaction_Log
                    WHERE user_id = p_user_id
                    ORDER BY timestamp;
                END;
                "#;

            sqlx::raw_sql(query).execute(pool).await?;

            let query =
                r#"
                CREATE FUNCTION IF NOT EXISTS GetMessageCount(
                    p_channel_id BIGINT
                ) RETURNS INT
                DETERMINISTIC
                BEGIN
                    DECLARE v_count INT;

                    SELECT COUNT(*) INTO v_count
                    FROM Message_Block
                    WHERE channel_id = p_channel_id;

                    RETURN v_count;
                END;
                "#;

            sqlx::raw_sql(query).execute(pool).await?;
        }

        Ok(())
    }

    pub async fn send_message(&self, user_id: i64, channel_id: i64, msg: &String)
        -> Result<(), sqlx::Error>
    {
        if let Some(pool) = &self.pool {
            let query = "CALL SendMessage(?, ?, ?);";

            sqlx::query(query)
                .bind(&user_id)
                .bind(&channel_id)
                .bind(&msg)
                .execute(pool)
                .await?;
        }

        Ok(())
    }

    pub async fn get_user_transactions(&self, user_id: i64) -> Result<(), sqlx::Error>
    {
        if let Some(pool) = &self.pool {
            let query = "CALL GetUserTransactions(?);";

            let rows = sqlx::query(query)
                .bind(&user_id)
                .fetch_all(pool)
                .await?;

            println!("{:#?}", rows);
        }

        Ok(())
    }
}
