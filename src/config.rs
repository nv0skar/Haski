// Haski - Oscar
// The use of this is restricted to only the authors

pub mod defaults {
    pub mod db {
        pub const DBPATH: &str = "./db";
    }

    pub mod trade {
        pub const PAIR: &str = "BTC-EUR";
        pub const START_DATE: &str = "2015-01-01";
        pub const END_DATE: &str = "2020-01-01";
    }

    pub mod learn {
        pub const PREVIOUS_VALUES: usize = 16;
        pub const FORWAD_VALUES: usize = 16;
        pub const PATTERN_THRESHOLD: usize = 15;
    }
}