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

use super::{ConnectionConfig, area::{Area, AreaDB}, dump_db, restore_db};
use std::collections::HashMap;
use sqlx::MySqlPool;

/// Global database manager.
#[derive(Debug, Default)]
pub struct GlobalDB {
    /// Area-specific database managers table.
    table: HashMap<Area, AreaDB>,
    /// Global MySQL connection pool.
    pool: Option<MySqlPool>,
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
        self.pool = Some(MySqlPool::connect(config.url_db().as_str()).await?);
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
        let config = &area_db.config();

        // Create database if not exists.
        if let Some(pool) = &self.pool {
            let query = format!(
                "CREATE DATABASE IF NOT EXISTS {}",
                config.database
            );

            sqlx::query(query.as_str()).execute(pool).await?;
        }
        else {
            // Handle case when connection pool was not initialized.
            eprintln!(
                "Error: connection pool is None: {}",
                "call GlobalDB::connect() method first!"
            );
            return Err(sqlx::Error::PoolClosed)
        }

        let mut manager = area_db;
        let url = config.url();
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
    pub fn dump_db_by_area(&self, area: &Area) -> Result<(), sqlx::Error> {
        // Check if area is correct.
        if !self.table.contains_key(area) {
            return Err(sqlx::Error::RowNotFound);
        }

        let area_db   = self.table.get(area).unwrap();
        let dump_name = format!("{}_backup.sql", area_db.name());

        dump_db(&area_db.config(), dump_name)?;
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
    pub fn restore_db_by_area(&self, area: &Area) -> Result<(), sqlx::Error> {
        // Check if area is correct.
        if !self.table.contains_key(area) {
            return Err(sqlx::Error::RowNotFound);
        }

        let area_db   = self.table.get(area).unwrap();
        let dump_name = format!("{}_backup.sql", area_db.name());

        restore_db(&area_db.config(), dump_name)?;
        Ok(())
    }
}