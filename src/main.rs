// Haski - Oscar
// The use of this is restricted to only the authors

#![allow(non_snake_case)]

mod io;
mod data;
mod trader;
mod cli;
mod utils;

use chrono::{Utc, NaiveDate, DateTime};

fn main() {
    let cli::argument::Args2Parse::Train(arguments) = cli::argument::parse(); {
        utils::show::printTitle("Haski");
        trader::heart::startLearning(arguments.startDate, arguments.endDate, arguments.pair, arguments.previousValues, arguments.forwadValues, arguments.patternThreshold);
    }
}
