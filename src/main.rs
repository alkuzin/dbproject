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

mod chat;
mod db;

use db::{ConnectionConfig, global::GlobalDB, area::{Area, AreaDB}};

/// Setup database managers.
///
/// # Returns
/// - `Ok` - in case of success.
/// - `sqlx::Error` - otherwise.
async fn setup_db() -> Result<(), sqlx::Error> {
    let global_config = ConnectionConfig::new(
        "test".to_string(),
        "12345".to_string(),
        "localhost".to_string(),
        3306,
        "".to_string(),
    );

    // Set global database manager.
    let mut global_db = GlobalDB::new();
    global_db.connect(&global_config).await?;

    let mut rus_area_config  = global_config.clone();
    rus_area_config.database = "AreaDB_Russia".to_string();

    // Set area-specific database managers.
    let russia_area_db = AreaDB::new(rus_area_config, Area::Russia);
    global_db.insert(russia_area_db).await?;

    let mut usa_area_config  = global_config.clone();
    usa_area_config.database = "AreaDB_USA".to_string();

    let usa_area_db = AreaDB::new(usa_area_config, Area::Usa);
    global_db.insert(usa_area_db).await?;

    global_db.test_procedures().await?;
    global_db.test_requests().await?;

    Ok(())
}

#[tokio::main]
async fn main() {
    if let Err(err) = setup_db().await {
        eprintln!("Error: {err}");
    }
}