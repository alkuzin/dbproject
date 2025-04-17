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

//! Main database declarations module.

use std::{process::Command, io};
use sqlx::MySqlPool;

pub mod global;
pub mod area;

/// MySQL connection config struct.
#[derive(Debug, Default, Clone)]
pub struct ConnectionConfig {
    /// MySQL username.
    pub username: String,
    /// MySQL user password.
    pub password: String,
    /// Connection host.
    pub host: String,
    /// Connection port.
    pub port: u16,
    /// MySQL database name.
    pub database: String,
}

impl ConnectionConfig {
    /// Construct new ConnectionConfig object.
    ///
    /// # Parameters
    /// - `username` - given MySQL username.
    /// - `password` - given MySQL user password.
    /// - `host`     - given connection host.
    /// - `port`     - given connection port.
    /// - `database` - given MySQL database name.
    ///
    /// # Returns
    /// - New `ConnectionConfig` object.
    pub fn new(
        username: String,
        password: String,
        host: String,
        port: u16,
        database: String
    ) -> Self {
        Self {
            username,
            password,
            host,
            port,
            database,
        }
    }

    /// Generate connection URL from config.
    ///
    /// # Returns
    /// - MySQL connection URL.
    pub fn url(&self) -> String {
        format!(
            "mysql://{}:{}@{}:{}",
            self.username,
            self.password,
            self.host,
            self.port
        )
    }

    /// Generate connection URL with database from config.
    ///
    /// # Returns
    /// - MySQL connection URL.
    pub fn url_db(&self) -> String {
        format!(
            "mysql://{}:{}@{}:{}/{}",
            self.username,
            self.password,
            self.host,
            self.port,
            self.database
        )
    }
}

/// CRUD (Create, Read, Update, Delete) operations trait.
pub trait CrudOps {
    async fn create(pool: &MySqlPool) -> Result<(), sqlx::Error>;
    async fn update(&self, pool: &MySqlPool) -> Result<(), sqlx::Error>;
    async fn delete(&self, pool: &MySqlPool) -> Result<(), sqlx::Error>;
    async fn fill_random(&mut self, pool: &MySqlPool) -> Result<(), sqlx::Error>;
}

/// Dump database into specific file.
///
/// # Parameters
/// - `config`   - given MySQL connection config.
/// - `filename` - given database dump file name.
///
/// # Returns
/// - `Ok`  - in case of success.
/// - `Err` - otherwise.
pub fn dump_db(config: &ConnectionConfig, filename: &String)
    -> Result<(), io::Error>
{
    let _ = Command::new("mysqldump")
        .arg("-u")
        .arg(config.username.as_str())
        .arg(format!("-p{}", config.password))
        .arg(config.database.as_str())
        .arg("--result-file")
        .arg(filename)
        .output()?;

    Ok(())
}

/// Restore database from specific file.
///
/// # Parameters
/// - `config`   - given MySQL connection config.
/// - `filename` - given database dump file name.
///
/// # Returns
/// - `Ok`  - in case of success.
/// - `Err` - otherwise.
pub fn restore_db(config: &ConnectionConfig, filename: &String)
    -> Result<(), io::Error>
{
    let _ = Command::new("mysql")
        .arg("-u")
        .arg(config.username.as_str())
        .arg(format!("-p{}", config.password))
        .arg(config.database.as_str())
        .arg("<")
        .arg(filename)
        .output()?;

    Ok(())
}

/// Create new table.
///
/// # Parameters
/// - `pool` - given MySQL connection pool.
/// - `name` - given table name.
///
/// # Returns
/// - `Ok` - in case of success.
/// - `sqlx::Error` - otherwise.
pub async fn create_table(pool: &MySqlPool, name: &String, content: &String)
    -> Result<(), sqlx::Error>
{
    let query = format!("CREATE TABLE IF NOT EXISTS {} ({});", name, content);
    sqlx::query(query.as_str()).execute(pool).await?;

    Ok(())
}

/// Create new database.
///
/// # Parameters
/// - `pool` - given MySQL connection pool.
/// - `name` - given database name.
///
/// # Returns
/// - `Ok` - in case of success.
/// - `sqlx::Error` - otherwise.
pub async fn create_db(pool: &MySqlPool, name: &String) -> Result<(), sqlx::Error> {
    // Create database if not exists.
    let query = format!("CREATE DATABASE IF NOT EXISTS {}", name);
    sqlx::query(query.as_str()).execute(pool).await?;

    Ok(())
}