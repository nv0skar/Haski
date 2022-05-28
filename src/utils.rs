// Haski
// Copyright (C) 2022 ItsTheGuy
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published
// by the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

pub mod show {
    use colored::Colorize;

    pub fn print(origin: &str, data: &String) {
        println!("{} {}", format!("({})", origin).bold().yellow(), data)
    }
    pub fn printError(origin: &str, data: &String) {
        println!("{} {}", format!("(ERROR at {})", origin).bold().red(), data)
    }
    pub fn printTitle(data: &str) {
        println!("{}", format!("--- {} ---", data).bright_white())
    }
}
