// Haski - Oscar
// The use of this is restricted to only the authors

pub mod argument {
    use clap::{ Parser, Args };

    #[derive(Parser, Debug)]
    #[clap(about = "Hash-powered trading bot")]
    pub enum Args2Parse {
       Train(Args2ParseLearner)
    }

    #[derive(Args, Debug)]
    pub struct Args2ParseLearner {
        #[clap(short, long, default_value = crate::config::defaults::trade::PAIR)]
        pub pair: String,
        #[clap(long, default_value = crate::config::defaults::trade::START_DATE)]
        pub startDate: String,
        #[clap(long, default_value = crate::config::defaults::trade::END_DATE)]
        pub endDate: String,
        #[clap(long, default_value_t = crate::config::defaults::learn::PREVIOUS_VALUES)]
        pub previousValues: usize,
        #[clap(long, default_value_t = crate::config::defaults::learn::FORWAD_VALUES)]
        pub forwadValues: usize,
        #[clap(long, default_value_t = crate::config::defaults::learn::PATTERN_THRESHOLD)]
        pub patternThreshold: usize,
    }

    pub fn parse() -> Args2Parse { Args2Parse::parse() }
}