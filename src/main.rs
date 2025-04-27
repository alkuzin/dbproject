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
mod task1;
mod task2;
mod task3;

#[tokio::main]
async fn main() {
    let task = 3;

    match task {
        1 => {
            if let Err(err) = task1::setup_db().await {
                eprintln!("Error: {err}");
            }
        }
        2 => {
            if let Err(err) = task2::setup_db().await {
                eprintln!("Error: {err}");
            }
        }
        3 => {
            if let Err(err) = task3::setup_db().await {
                eprintln!("Error: {err}");
            }
        }
        _ => {
            eprintln!("Incorrect task: {task}");
        }
    }
}