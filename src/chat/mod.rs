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

//! Main module for chat related structs.

mod user;
mod channel;
mod message;
mod bans;
mod settings;
mod logs;

pub use user::{User, ChannelUser, UserProfile};
pub use channel::Channel;
pub use message::{Message, Reaction};
pub use bans::Ban;
pub use settings::{
    ServerSetting, ChannelSetting, UserSetting,
    ChannelSettingKV, ServerSettingKV, UserSettingKV
};
pub use logs::Log;
use crate::db::CrudOps;
use sqlx::MySqlPool;

/// Create database tables.
///
/// # Parameters
/// - `pool` - given MySQL connection pool.
///
/// # Returns
/// - `Ok` - in case of success.
/// - `sqlx::Error` - otherwise.
pub async fn create_db_tables(pool: &MySqlPool) -> Result<(), sqlx::Error> {
    User::create(pool).await?;
    Channel::create(pool).await?;
    Message::create(pool).await?;
    ChannelUser::create(pool).await?;
    Ban::create(pool).await?;
    ServerSetting::create(pool).await?;
    Log::create(pool).await?;
    UserProfile::create(pool).await?;
    Reaction::create(pool).await?;
    ChannelSetting::create(pool).await?;
    UserSetting::create(pool).await?;

    ServerSettingKV::create(pool).await?;
    ChannelSettingKV::create(pool).await?;
    UserSettingKV::create(pool).await?;

    Ok(())
}

/// Fill database tables with random entries.
///
/// # Parameters
/// - `pool` - given MySQL connection pool.
///
/// # Returns
/// - `Ok` - in case of success.
/// - `sqlx::Error` - otherwise.
pub async fn fill_db_tables(pool: &MySqlPool, count: u32) -> Result<(), sqlx::Error> {
    let mut user            = User::default();
    let mut channel         = Channel::default();
    let mut message         = Message::default();
    let mut channel_user    = ChannelUser::default();
    let mut ban             = Ban::default();
    let mut server_setting  = ServerSetting::default();
    let mut log             = Log::default();
    let mut user_profile    = UserProfile::default();
    let mut reaction        = Reaction::default();
    let mut channel_setting = ChannelSetting::default();
    let mut user_setting    = UserSetting::default();

    let mut server_setting_kv  = ServerSettingKV::default();
    let mut channel_setting_kv = ChannelSettingKV::default();
    let mut user_setting_kv    = UserSettingKV::default();

    for _ in 0..count {
        user.fill_random(pool).await?;
        channel.fill_random(pool).await?;
        message.fill_random(pool).await?;
        channel_user.fill_random(pool).await?;
        ban.fill_random(pool).await?;
        server_setting.fill_random(pool).await?;
        log.fill_random(pool).await?;
        user_profile.fill_random(pool).await?;
        reaction.fill_random(pool).await?;
        channel_setting.fill_random(pool).await?;
        user_setting.fill_random(pool).await?;

        server_setting_kv.fill_random(pool).await?;
        channel_setting_kv.fill_random(pool).await?;
        user_setting_kv.fill_random(pool).await?;
    }

    Ok(())
}