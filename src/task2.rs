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

use crate::{db::{ConnectionConfig, docdb::DocDBManager}};
use crate::chat::UserProfile;

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
        "DocumentDB_JSON".to_string(),
    );

    let mut doc_db_manager = DocDBManager::new();
    doc_db_manager.connect(config).await?;
    doc_db_manager.set_procedures().await?;

    let user_profile = UserProfile {
        profile_id:          5,
        user_id:             0,
        bio:                 "Very cool user".to_string(),
        profile_picture_url: "https://example.com/sdfcgvhjksome_image.png".to_string(),
        location:            "Russia ...".to_string(),
        ..Default::default()
    };
    doc_db_manager.add_user_profile_data(&user_profile).await?;
    let updated_user_profile = doc_db_manager.get_user_profile_data(10).await?;

    println!("{:#?}", updated_user_profile);
    doc_db_manager.test_trigger(&user_profile).await?;

    Ok(())
}