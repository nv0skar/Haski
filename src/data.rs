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

use chrono::{DateTime, Utc};
use time::OffsetDateTime;
use tokio_test;
use yahoo_finance_api as dataProvider;

pub fn retrieve(
    start: DateTime<Utc>,
    end: DateTime<Utc>,
    pair: &String,
) -> Vec<dataProvider::Quote> {
    let provider = dataProvider::YahooConnector::new();
    let resp = tokio_test::block_on(provider.get_quote_history(
        &pair,
        OffsetDateTime::from_unix_timestamp(start.timestamp()).unwrap(),
        OffsetDateTime::from_unix_timestamp(end.timestamp()).unwrap(),
    ))
    .unwrap();
    resp.quotes().unwrap()
}

pub fn maxValue(data: &Vec<f64>) -> f64 {
    let mut biggestValue: f64 = 0.0;
    for number in data {
        if biggestValue < number.clone() {
            biggestValue = number.clone()
        }
    }
    biggestValue
}
