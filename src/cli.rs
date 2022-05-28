// Haski - Oscar
// The use of this is restricted to only the authors

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
            #[clap(long, default_value_t = crate::config::defaults::learn::FORWAD_VALUES)]
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
        },
    }

    pub fn parse() -> Command {
        Command::parse()
    }
}
