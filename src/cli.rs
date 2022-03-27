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
        #[clap(long, default_value_t = 16)]
        pub previous: usize,
        #[clap(long, default_value_t = 16)]
        pub forward: usize,
        #[clap(long, default_value_t = 15)]
        pub threshold: usize,
        #[clap(long, default_value_t = 2015)]
        pub startYear: usize,
        #[clap(long, default_value_t = 01)]
        pub startMonth: usize,
        #[clap(long, default_value_t = 01)]
        pub startDay: usize,
        #[clap(long, default_value_t = 2020)]
        pub endYear: usize,
        #[clap(long, default_value_t = 01)]
        pub endMonth: usize,
        #[clap(long, default_value_t = 01)]
        pub endDay: usize,
    }

    pub fn parse() -> Args2Parse { Args2Parse::parse() }
}