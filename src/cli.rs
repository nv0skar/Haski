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
        #[clap(short, long, default_value = "BTC-EUR")]
        pub pair: String,
        #[clap(long, default_value = "2015-01-01")]
        pub startDate: String,
        #[clap(long, default_value = "2020-01-01")]
        pub endDate: String,
        #[clap(long, default_value_t = 16)]
        pub previousValues: usize,
        #[clap(long, default_value_t = 16)]
        pub forwadValues: usize,
        #[clap(long, default_value_t = 15)]
        pub patternThreshold: usize,
    }

    pub fn parse() -> Args2Parse { Args2Parse::parse() }
}