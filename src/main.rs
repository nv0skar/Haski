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

#![allow(non_snake_case)]

mod cli;
mod config;
mod data;
mod db;
mod plotter;
mod trader;
mod utils;

use crate::utils::Print;

use std::collections::HashMap;

fn main() {
    let parsed = cli::parser::parse();

    Print::show(
        Print::PrintType::Beauty,
        None,
        env!("CARGO_PKG_NAME").to_string(),
    );

    let mut db = db::Database::default();
    db.open(parsed.dbLocation);
    match parsed.command {
        cli::parser::Subcommands::Train {
            pair,
            startDate,
            endDate,
            previousValues,
            forwadValues,
            patternThreshold,
        } => {
            trader::train(
                &mut db,
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
            initialBalance,
            tradeAmount,
            stopLoss,
            takeProfit,
        } => {
            trader::backtest(
                &mut db,
                startDate,
                endDate,
                pair,
                initialBalance,
                tradeAmount,
                stopLoss,
                takeProfit,
            );
        }
    }
}
