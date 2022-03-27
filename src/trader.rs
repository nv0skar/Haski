// Haski - Oscar
// The use of this is restricted to only the authors

use chrono::{DateTime, Utc};
use wyhash::WyHash;
use yahoo_finance_api as dataProvider;
use tokio_test;

pub mod dataFetcher { 
    pub fn retrieve(start: super::DateTime<super::Utc>, end: super::DateTime<super::Utc>) -> Vec<super::dataProvider::Quote> {
        let provider = super::dataProvider::YahooConnector::new();
        let resp = super::tokio_test::block_on(provider.get_quote_history("BTC-EUR", start, end)).unwrap();
        resp.quotes().unwrap()
    }
}

pub mod heart {
    use std::hash::Hasher;

    #[derive(Debug)]
    pub enum Actions {
        Buy,
        Sell,
        Hold
    }

    pub fn startLearning(data: Vec<super::dataProvider::Quote>, lookBack: usize, lookForward: usize, patternThreshold: usize) {
        let db = crate::io::file::openDB().unwrap();
        
        let _ = crate::io::file::writeConfig(&db, lookBack, lookForward, patternThreshold);
        
        let mut patterns: Vec<usize> = vec![];
        let mut patternsAction: Vec<Actions> = vec![];

        for itemNum in 0..data.len() {
            if (itemNum < (lookBack+1)) || ((data.len()-itemNum) < lookForward) { continue }
            let mut sumForwardValues: f64 = 0.0;
            for itemNumForward in itemNum..(lookForward+itemNum) { sumForwardValues += data[itemNumForward].close }
            let averageForwardValues = sumForwardValues / (lookForward as f64);

            let priceDeviation = ((averageForwardValues / data[itemNum].close) * 100 as f64) - 100 as f64;

            if priceDeviation.abs() >= patternThreshold as f64 { 
                patterns.push(itemNum);
                if priceDeviation > 0 as f64 { patternsAction.push(Actions::Buy) } else if priceDeviation < 0 as f64 { patternsAction.push(Actions::Sell)}
                else { patternsAction.push(Actions::Hold) }
            }
        }

        for itemNum in 0..patterns.len() {
            let mut patternValueDerivation: Vec<u8> = vec![];
            for itemNumDevCalc in (patterns[itemNum]-lookBack)..(patterns[itemNum]) {
                let valueDerivation = (((((data[itemNumDevCalc].close / data[itemNumDevCalc-1].close) * 100 as f64) - 100 as f64).abs()).ln()).round() as u8;
                patternValueDerivation.push(valueDerivation)
            }
            
            let mut hash = super::WyHash::with_seed(0);
            hash.write(&patternValueDerivation);

            let calculatedHash = hash.finish();
            let _ = crate::io::file::writePattern(&db, calculatedHash, &patternsAction[itemNum]);
            crate::utils::show::print("Learner", &format!("Pattern found! Hash: {}; Signal: {:?}", calculatedHash, patternsAction[itemNum]))
        }
    }
}