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
pub use user::User;

// TODO: implement structs for these tables:

// CREATE TABLE User (
// user_id BIGINT AUTO_INCREMENT UNIQUE,
// username TINYTEXT,
// password_hash LONGTEXT,
// email TEXT,
// created_at DATE,
// last_login DATE,
// PRIMARY KEY(user_id)
// );

// CREATE TABLE Channel (
// channel_id BIGINT AUTO_INCREMENT UNIQUE,
// channel_name TINYTEXT,
// topic INTEGER,
// created_by BIGINT,
// creator DATE,
// is_private BOOLEAN,
// PRIMARY KEY(channel_id)
// );
//
//
// CREATE TABLE Message (
// message_id BIGINT AUTO_INCREMENT UNIQUE,
// channel_id BIGINT,
// user_id BIGINT,
// message_text TEXT,
// timestamp DATE,
// PRIMARY KEY(message_id)
// );
//
//
// CREATE TABLE Channel_Users (
// channel_user_id BIGINT,
// channel_id BIGINT AUTO_INCREMENT UNIQUE,
// user_id BIGINT,
// joined_at DATE,
// role TEXT,
// PRIMARY KEY(channel_user_id)
// );
//
//
// CREATE TABLE Bans (
// ban_id INTEGER AUTO_INCREMENT UNIQUE,
// channel_id BIGINT,
// user_id BIGINT,
// banned_at DATE,
// reason TEXT,
// PRIMARY KEY(ban_id)
// );
//
//
// CREATE TABLE Server_Settings (
// setting_id BIGINT AUTO_INCREMENT UNIQUE,
// settings_name TEXT,
// settings_value INTEGER,
// PRIMARY KEY(setting_id)
// );
//
//
// CREATE TABLE Logs (
// log_id BIGINT AUTO_INCREMENT UNIQUE,
// event_type TEXT,
// user_id BIGINT,
// channel_id BIGINT,
// timestamp DATE,
// details TEXT,
// PRIMARY KEY(log_id)
// );
//
//
// CREATE TABLE User_Profiles (
// profile_id BIGINT AUTO_INCREMENT UNIQUE,
// user_id BIGINT,
// bio TEXT,
// profile_picture_url TEXT,
// location TEXT,
// PRIMARY KEY(profile_id)
// );
//
//
// CREATE TABLE Reactions (
// reaction_id BIGINT AUTO_INCREMENT UNIQUE,
// message_id BIGINT,
// user_id BIGINT,
// timestamp DATE,
// reaction_type VARCHAR(255),
// PRIMARY KEY(reaction_id)
// );
//
//
// CREATE TABLE Channel_Settings (
// setting_id BIGINT AUTO_INCREMENT UNIQUE,
// channel_id BIGINT,
// setting_name TEXT,
// setting_value TEXT,
// PRIMARY KEY(setting_id)
// );
//
//
// CREATE TABLE User_Settings (
// settings_id BIGINT AUTO_INCREMENT UNIQUE,
// user_id BIGINT,
// settings_name TEXT,
// settings_value TEXT,
// PRIMARY KEY(settings_id)
// );
