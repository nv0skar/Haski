// Haski - Oscar
// The use of this is restricted to only the authors

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
