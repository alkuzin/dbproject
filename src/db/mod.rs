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

use sqlx::MySqlPool;

pub mod global;
pub mod area;

/// MySQL connection config struct.
#[derive(Debug, Default, Copy, Clone)]
pub struct ConnectionConfig {
    /// MySQL username.
    pub username: &'static str,
    /// MySQL user password.
    pub password: &'static str,
    /// Connection host.
    pub host: &'static str,
    /// Connection port.
    pub port: u16,
    /// MySQL database name.
    pub database: &'static str,
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
        username: &'static str,
        password: &'static str,
        host: &'static str,
        port: u16,
        database: &'static str
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
    fn create(&self, pool: &MySqlPool) -> Result<(), sqlx::Error>;
    fn update(&self, pool: &MySqlPool) -> Result<(), sqlx::Error>;
    fn delete(&self, pool: &MySqlPool) -> Result<(), sqlx::Error>;
}