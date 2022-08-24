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

pub mod parser {
    use clap::{Parser, Subcommand};

    #[derive(Parser, Debug)]
    #[clap(about = "Hash-powered trading bot")]
    pub struct Command {
        #[clap(subcommand)]
        pub command: Subcommands,
        #[clap(long, default_value = crate::config::defaults::db::PATH)]
        pub dbLocation: String,
    }

    #[derive(Subcommand, Debug)]
    pub enum Subcommands {
        #[clap(about = "Train the bot with data from Yahoo Finance")]
        Train {
            #[clap(short, long, default_value = crate::config::defaults::trade::PAIR)]
            pair: String,
            #[clap(long, default_value = crate::config::defaults::trade::START_DATE)]
            startDate: String,
            #[clap(long, default_value = crate::config::defaults::trade::END_DATE)]
            endDate: String,
            #[clap(long, default_value_t = crate::config::defaults::learn::PREVIOUS_VALUES)]
            previousValues: usize,
            #[clap(long, default_value_t = crate::config::defaults::learn::FORWARD_VALUES)]
            forwadValues: usize,
            #[clap(long, default_value_t = crate::config::defaults::learn::PATTERN_THRESHOLD)]
            patternThreshold: usize,
        },
        #[clap(about = "Backtest the bot with the training data")]
        Backtest {
            #[clap(short, long, default_value = crate::config::defaults::trade::PAIR)]
            pair: String,
            #[clap(long, default_value = crate::config::defaults::trade::START_DATE)]
            startDate: String,
            #[clap(long, default_value = crate::config::defaults::trade::END_DATE)]
            endDate: String,
            #[clap(long, default_value_t = crate::config::defaults::trade::INITIAL_BALANCE)]
            initialBalance: f64,
            #[clap(long, default_value_t = crate::config::defaults::trade::TRADE_AMOUNT)]
            tradeAmount: f64,
            #[clap(long, default_value_t = crate::config::defaults::trade::STOP_LOSS)]
            stopLoss: f64,
            #[clap(long, default_value_t = crate::config::defaults::trade::TAKE_PROFIT)]
            takeProfit: f64,
        },
    }

    pub fn parse() -> Command {
        Command::parse()
    }
}
