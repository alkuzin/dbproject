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

use crate::{db::{ConnectionConfig, blockchain::BlockchainDBManager}};

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
        "BlockchainDB".to_string(),
    );

    let mut blockchain_db_manager = BlockchainDBManager::new();
    blockchain_db_manager.connect(config).await?;
    blockchain_db_manager.set_procedures().await?;

    blockchain_db_manager.send_message(1, 1, &"Some test message1".to_string()).await?;
    blockchain_db_manager.send_message(1, 1, &"Some test message2".to_string()).await?;
    blockchain_db_manager.get_user_transactions(1).await?;

    Ok(())
}
