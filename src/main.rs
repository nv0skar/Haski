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

#![allow(non_snake_case)]

mod cli;
mod config;
mod data;
mod io;
mod plotter;
mod trader;
mod utils;

use chrono::{Date, DateTime, Local, NaiveDate, Utc};
use std::collections::HashMap;

fn main() {
    let parsed = cli::parser::parse();
    utils::show::printTitle("Haski");
    match parsed.command {
        cli::parser::Subcommands::Train {
            pair,
            startDate,
            endDate,
            previousValues,
            forwadValues,
            patternThreshold,
        } => {
            trader::heart::startLearning(
                parsed.dbLocation,
                startDate,
                endDate,
                pair,
                previousValues,
                forwadValues,
                patternThreshold,
            );
        }
        cli::parser::Subcommands::Backtest {
            pair,
            startDate,
            endDate,
        } => {
            trader::heart::backtest(parsed.dbLocation, startDate, endDate, pair);
        }
    }
}
