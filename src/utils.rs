// Haski
// Copyright (C) 2022 Oscar
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

pub mod Convert {
    use chrono::{Local, NaiveDate, TimeZone};

    pub fn fromTimestamp2Date(timestamp: u64) -> NaiveDate {
        Local
            .timestamp_opt(timestamp as i64, 0)
            .unwrap()
            .date_naive()
    }
}

pub mod Print {
    use colored::Colorize;

    #[derive(PartialEq)]
    pub enum PrintType {
        Beauty,
        Info,
        Error,
    }

    pub fn show(kind: PrintType, src: Option<String>, data: String) {
        if kind != PrintType::Beauty && src.is_none() {
            panic!("Tried to show message, but no source was provided!")
        }
        match kind {
            PrintType::Beauty => println!("{}", format!("--- {} ---", data).bright_white()),
            PrintType::Info => {
                println!("{} {}", format!("({})", src.unwrap()).yellow().bold(), data)
            }
            PrintType::Error => {
                println!(
                    "{} {}",
                    format!("(ERROR at {})", src.unwrap()).bold().red(),
                    data
                );
                panic!("{}", data);
            }
        }
    }
}
