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

use tokio_test;
use yahoo_finance_api as dataProvider;

pub mod fetcher {
    pub fn retrieve(
        start: crate::DateTime<crate::Utc>,
        end: crate::DateTime<crate::Utc>,
        pair: &String,
    ) -> Vec<super::dataProvider::Quote> {
        let provider = super::dataProvider::YahooConnector::new();
        let resp =
            super::tokio_test::block_on(provider.get_quote_history(&pair, start, end)).unwrap();
        resp.quotes().unwrap()
    }
}
