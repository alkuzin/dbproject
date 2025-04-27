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

use crate::{db::{ConnectionConfig, kvdb::KeyValueDBManager}};
use crate::chat::UserSettingKV;

/// Setup database managers.
///
/// # Returns
/// - `Ok` - in case of success.
/// - `sqlx::Error` - otherwise.
pub async fn setup_db() -> Result<(), sqlx::Error> {
    let config = ConnectionConfig::new(
        "test".to_string(),
        "12345".to_string(),
        "localhost".to_string(),
        3306,
        "KeyValueDB".to_string(),
    );

    let mut kv_db_manager = KeyValueDBManager::new();
    kv_db_manager.connect(config).await?;
    kv_db_manager.set_procedures().await?;

    let user_settings_kv = UserSettingKV {
        user_id: 1337,
        settings_name:  "theme".to_string(),
        settings_value: "dark".to_string(),
    };

    kv_db_manager.add_user_setting(&user_settings_kv).await?;

    // let setting_name = kv_db_manager.get_user_setting(
    //     user_settings_kv.user_id, user_settings_kv.settings_name
    // ).await?;
    // println!("GetUserSetting: {setting_name}");

    match kv_db_manager.get_all_user_settings(1337).await {
        Ok(Some(settings)) => println!("User settings: {}", settings),
        Ok(None)           => println!("No settings found for the user."),
        Err(e)             => eprintln!("Error: {}", e),
    }

    Ok(())
}
