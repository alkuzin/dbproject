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

//! Settings related structs.

use crate::db::{create_table, CrudOps};
use sqlx::MySqlPool;
use rand::Rng;

/// ServerSetting table.
#[derive(Debug, Default)]
pub struct ServerSetting {
    /// Setting identifier.
    setting_id: i64,
    /// Name of the setting.
    settings_name: String,
    /// Value of the setting.
    settings_value: i32,
}

impl CrudOps for ServerSetting {
    async fn create(pool: &MySqlPool) -> Result<(), sqlx::Error> {
        let name = "Server_Settings".to_string();
        let content = String::from(
            r#"
            setting_id BIGINT AUTO_INCREMENT UNIQUE,
            settings_name TEXT,
            settings_value INTEGER,
            PRIMARY KEY(setting_id)
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
        self.settings_name = format!("setting_{}", rng.gen_range(1..10000));
        self.settings_value = rng.gen_range(1..100); // Random integer value for the setting.

        // Insert the new server setting into the database.
        sqlx::query(
            r#"
            INSERT INTO Server_Settings
            (settings_name, settings_value)
            VALUES (?, ?)
            "#,
        )
            .bind(&self.settings_name)
            .bind(self.settings_value)
            .execute(pool)
            .await?;

        Ok(())
    }
}

/// ChannelSetting table.
#[derive(Debug, Default)]
pub struct ChannelSetting {
    /// Setting identifier.
    setting_id: i64,
    /// Channel identifier associated with the setting.
    channel_id: i64,
    /// Name of the setting.
    setting_name: String,
    /// Value of the setting.
    setting_value: String,
}

impl CrudOps for ChannelSetting {
    async fn create(pool: &MySqlPool) -> Result<(), sqlx::Error> {
        let name = "Channel_Settings".to_string();
        let content = String::from(
            r#"
            setting_id BIGINT AUTO_INCREMENT UNIQUE,
            channel_id BIGINT,
            setting_name TEXT,
            setting_value TEXT,
            PRIMARY KEY(setting_id)
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
        self.setting_name = format!("setting_name_{}", rng.gen_range(1..100));
        self.setting_value = format!("setting_value_{}", rng.gen_range(1..100)); // Random value for the setting.

        // Insert the new channel setting into the database.
        sqlx::query(
            r#"
            INSERT INTO Channel_Settings
            (channel_id, setting_name, setting_value)
            VALUES (?, ?, ?)
            "#,
        )
            .bind(self.channel_id)
            .bind(&self.setting_name)
            .bind(&self.setting_value)
            .execute(pool)
            .await?;

        Ok(())
    }
}

/// UserSetting table.
#[derive(Debug, Default)]
pub struct UserSetting {
    /// Setting identifier.
    settings_id: i64,
    /// User identifier associated with the setting.
    user_id: i64,
    /// Name of the setting.
    settings_name: String,
    /// Value of the setting.
    settings_value: String,
}

impl CrudOps for UserSetting {
    async fn create(pool: &MySqlPool) -> Result<(), sqlx::Error> {
        let name = "User_Settings".to_string();
        let content = String::from(
            r#"
            settings_id BIGINT AUTO_INCREMENT UNIQUE,
            user_id BIGINT,
            settings_name TEXT,
            settings_value TEXT,
            PRIMARY KEY(settings_id)
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
        self.settings_name = format!("setting_name_{}", rng.gen_range(1..100));
        self.settings_value = format!("setting_value_{}", rng.gen_range(1..100)); // Random value for the setting.

        // Insert the new user setting into the database.
        sqlx::query(
            r#"
            INSERT INTO User_Settings
            (user_id, settings_name, settings_value)
            VALUES (?, ?, ?)
            "#,
        )
            .bind(self.user_id)
            .bind(&self.settings_name)
            .bind(&self.settings_value)
            .execute(pool)
            .await?;

        Ok(())
    }
}





/// ServerSettingKV table.
#[derive(Debug, Default)]
pub struct ServerSettingKV {
    /// Name of the setting.
    settings_name: String,
    /// Value of the setting.
    settings_value: String,
}

impl CrudOps for ServerSettingKV {
    async fn create(pool: &MySqlPool) -> Result<(), sqlx::Error> {
        let name = "Server_Settings_KV".to_string();
        let content = String::from(
            r#"
            setting_name VARCHAR(255),
            setting_value VARCHAR(255),
            PRIMARY KEY(setting_name)
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
        self.settings_name  = format!("setting_{}", rng.gen_range(1..10000));
        self.settings_value = format!("setting_val_{}", rng.gen_range(1..10000));

        // Insert the new server setting into the database.
        sqlx::query(
            r#"
            INSERT INTO Server_Settings_KV
            (setting_name, setting_value)
            VALUES (?, ?)
            "#,
        )
            .bind(&self.settings_name)
            .bind(&self.settings_value)
            .execute(pool)
            .await?;

        Ok(())
    }
}

/// ChannelSettingKV table.
#[derive(Debug, Default)]
pub struct ChannelSettingKV {
    /// Channel identifier.
    channel_id: i64,
    /// Name of the setting.
    settings_name: String,
    /// Value of the setting.
    settings_value: String,
}

impl CrudOps for ChannelSettingKV {
    async fn create(pool: &MySqlPool) -> Result<(), sqlx::Error> {
        let name = "Channel_Settings_KV".to_string();
        let content = String::from(
            r#"
            channel_id BIGINT AUTO_INCREMENT UNIQUE,
            setting_name VARCHAR(255),
            setting_value VARCHAR(255),
            PRIMARY KEY(channel_id, setting_name)
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

    async fn fill_random(&mut self, pool: &MySqlPool) -> Result<(), sqlx::Error> {
        let mut rng = rand::thread_rng();

        // Generate random values.
        self.channel_id     = rng.gen_range(1..100);
        self.settings_name  = format!("setting_{}", rng.gen_range(1..10000));
        self.settings_value = format!("setting_val_{}", rng.gen_range(1..10000));

        // Insert the new server setting into the database.
        sqlx::query(
            r#"
            INSERT INTO Channel_Settings_KV
            (channel_id, setting_name, setting_value)
            VALUES (?, ?, ?)
            "#,
        )
            .bind(&self.channel_id)
            .bind(&self.settings_name)
            .bind(&self.settings_value)
            .execute(pool)
            .await?;

        Ok(())
    }
}

/// UserSettingKV table.
#[derive(Debug, Default)]
pub struct UserSettingKV {
    /// Channel identifier.
    pub user_id: i64,
    /// Name of the setting.
    pub settings_name: String,
    /// Value of the setting.
    pub settings_value: String,
}

impl CrudOps for UserSettingKV {
    async fn create(pool: &MySqlPool) -> Result<(), sqlx::Error> {
        let name = "User_Settings_KV".to_string();
        let content = String::from(
            r#"
            user_id BIGINT AUTO_INCREMENT UNIQUE,
            setting_name VARCHAR(255),
            setting_value VARCHAR(255),
            PRIMARY KEY(user_id, setting_name)
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

    async fn fill_random(&mut self, pool: &MySqlPool) -> Result<(), sqlx::Error> {
        let mut rng = rand::thread_rng();

        // Generate random values.
        self.user_id        = rng.gen_range(1..100);
        self.settings_name  = format!("setting_{}", rng.gen_range(1..10000));
        self.settings_value = format!("setting_val_{}", rng.gen_range(1..10000));

        // Insert the new server setting into the database.
        sqlx::query(
            r#"
            INSERT INTO User_Settings_KV
            (user_id, setting_name, setting_value)
            VALUES (?, ?, ?)
            "#,
        )
            .bind(&self.user_id)
            .bind(&self.settings_name)
            .bind(&self.settings_value)
            .execute(pool)
            .await?;

        Ok(())
    }
}
