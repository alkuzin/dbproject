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

use crate::db::CrudOps;
use sqlx::MySqlPool;

// CREATE TABLE User (
// user_id BIGINT AUTO_INCREMENT UNIQUE,
// username TINYTEXT,
// password_hash LONGTEXT,
// email TEXT,
// created_at DATE,
// last_login DATE,
// PRIMARY KEY(user_id)
// );

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
    created_at: Option<chrono::NaiveDate>,
    /// User last login time.
    last_login: Option<chrono::NaiveDate>,
}

// TODO: complete.
impl CrudOps for User {
    fn create(&self, _pool: &MySqlPool) -> Result<(), sqlx::Error> {
        todo!()
    }

    fn update(&self, _pool: &MySqlPool) -> Result<(), sqlx::Error> {
        todo!()
    }

    fn delete(&self, _pool: &MySqlPool) -> Result<(), sqlx::Error> {
        todo!()
    }
}
