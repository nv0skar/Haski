// Haski - Oscar
// The use of this is restricted to only the authors

#![allow(non_snake_case)]

mod io;
mod trader;
mod cli;
mod utils;

use chrono::{Utc,TimeZone};

fn main() {
    let cli::argument::Args2Parse::Train(arguments) = cli::argument::parse(); {
        utils::show::printTitle("Haski");
        let start = Utc.ymd(arguments.startYear as i32, arguments.startMonth as u32, arguments.startDay as u32).and_hms_milli(0, 0, 0, 0);
        let end = Utc.ymd(arguments.endYear as i32, arguments.endMonth as u32, arguments.endDay as u32).and_hms_milli(23, 59, 59, 999);
        let data = trader::dataFetcher::retrieve(start, end);
        trader::heart::startLearning(data, arguments.previous, arguments.forward, arguments.threshold);
    }
}
