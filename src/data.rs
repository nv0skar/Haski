// Haski - Oscar
// The use of this is restricted to only the authors

use yahoo_finance_api as dataProvider;
use tokio_test;


pub mod fetcher { 
    pub fn retrieve(start: crate::DateTime<crate::Utc>, end: crate::DateTime<crate::Utc>, pair: &String) -> Vec<super::dataProvider::Quote> {
        let provider = super::dataProvider::YahooConnector::new();
        let resp = super::tokio_test::block_on(provider.get_quote_history(&pair, start, end)).unwrap();
        resp.quotes().unwrap()
    }
}