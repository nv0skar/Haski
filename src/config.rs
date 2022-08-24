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

pub mod defaults {
    pub mod db {
        pub const PATH: &str = "./training";
    }

    pub mod trade {
        pub const PAIR: &str = "BTC-EUR";
        pub const START_DATE: &str = "2015-01-01";
        pub const END_DATE: &str = "2020-01-01";
        pub const INITIAL_BALANCE: f64 = 1000.0;
        pub const TRADE_AMOUNT: f64 = 50.0;
        pub const STOP_LOSS: f64 = 1.0;
        pub const TAKE_PROFIT: f64 = 10.0;
    }

    pub mod learn {
        pub const PREVIOUS_VALUES: usize = 16;
        pub const FORWARD_VALUES: usize = 16;
        pub const PATTERN_THRESHOLD: usize = 15;
    }
}
