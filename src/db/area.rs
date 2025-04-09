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

//! Area-specific database manager related declarations.

use crate::chat::User;
use sqlx::MySqlPool;

/// Area enumeration.
#[derive(Debug, Default, Hash, Eq, PartialEq, Copy, Clone)]
pub enum Area {
    #[default]
    Unknown,
    Russia,
}

/// Area-specific database manager.
#[derive(Debug, Default)]
pub struct AreaDB {
    /// Manager MySQL connection pool.
    pool: Option<MySqlPool>,
    /// Manager name.
    name: &'static str,
    /// Manager area.
    area: Area,
    // TODO: implement Tables struct to contain chat tables
    /// "User" database table.
    user_table: User,
}

impl AreaDB {
    /// Construct new AreaDB object.
    ///
    /// # Parameters
    /// - `name` - given manager name.
    /// - `area` - given manager area.
    ///
    /// # Returns
    /// - New `AreaDB` object.
    pub fn new(name: &'static str, area: Area) -> Self {
        let mut area_db = AreaDB::default();

        area_db.name = name;
        area_db.area = area;
        area_db
    }

    /// Connect database.
    ///
    /// # Parameters
    /// - `url` - given database connection URL.
    ///
    /// # Returns
    /// - `Ok` - in case of success.
    /// - `sqlx::Error` - otherwise.
    pub async fn connect(&mut self, url: &str) -> Result<(), sqlx::Error> {
        self.pool = Some(MySqlPool::connect(url).await?);
        Ok(())
    }

    /// Get manager area.
    ///
    /// # Returns
    /// - Manager area enumeration value.
    #[inline(always)]
    pub fn area(&self) -> Area {
        self.area
    }
}
