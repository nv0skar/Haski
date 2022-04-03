// Haski - Oscar
// The use of this is restricted to only the authors

use wyhash::WyHash;

pub mod heart {
    use std::{hash::Hasher, process::exit};

    #[derive(Debug)]
    pub enum Actions {
        Buy,
        Sell,
        Hold
    }

    pub fn startLearning(startDate: String, endDate: String, pair: String, lookBack: usize, lookForwad: usize, patternThreshold: usize) {
        let data = crate::data::fetcher::retrieve(crate::DateTime::<crate::Utc>::from_utc(crate::NaiveDate::parse_from_str(&startDate, "%Y-%m-%d").unwrap().and_hms(0, 0, 0), crate::Utc), crate::DateTime::<crate::Utc>::from_utc(crate::NaiveDate::parse_from_str(&endDate, "%Y-%m-%d").unwrap().and_hms(23, 59, 59), crate::Utc), &pair);

        let db = crate::io::file::hashPatterns::openDB().unwrap();

        let storedConfig = crate::io::file::hashPatterns::getConfig(&db);
        if let Some(value) = storedConfig.ClB.unwrap() { 
            if ((value[7] as usize) != lookBack) && (lookBack != crate::config::defaults::learn::PREVIOUS_VALUES) {
                drop(&db);
                crate::utils::show::printError("Learner", &format!("Previous values to get value is different from the one in the database! (Value: {})", (value[7] as usize)));
                exit(1);
            }
        }
        if let Some(value) = storedConfig.ClF.unwrap() {
            if ((value[7] as usize) != lookForwad) && (lookForwad != crate::config::defaults::learn::FORWAD_VALUES) {
                drop(&db);
                crate::utils::show::printError("Learner", &format!("Forwad values to get value is different from the one in the database! (Value: {})", (value[7] as usize)));
                exit(1);
            }
        }
        if let Some(value) = storedConfig.CpT.unwrap() {
            if ((value[7] as usize) != patternThreshold) && (patternThreshold != crate::config::defaults::learn::PATTERN_THRESHOLD) {
                drop(&db);
                crate::utils::show::printError("Learner", &format!("Pattern Threshold value is different from the one in the database! (Value: {})", (value[7] as usize)));
                exit(1);
            }
        }

        let _ = crate::io::file::hashPatterns::writeConfig(&db, &pair, lookBack, lookForwad, patternThreshold);

        let mut patternsFound: usize = 0;
        
        let mut patterns: Vec<usize> = vec![];
        let mut patternsAction: Vec<Actions> = vec![];

        for itemNum in 0..data.len() {
            if (itemNum < (lookBack+1)) || ((data.len()-itemNum) < lookForwad) { continue }
            let mut sumForwardValues: f64 = 0.0;
            for itemNumForward in itemNum..(lookForwad+itemNum) { sumForwardValues += data[itemNumForward].close }
            let averageForwardValues = sumForwardValues / (lookForwad as f64);

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
            let _ = crate::io::file::hashPatterns::writePattern(&db, calculatedHash, &patternsAction[itemNum]);
            patternsFound += 1;
            crate::utils::show::print("Learner", &format!("#{} pattern found! Hash: {}; Signal: {:?}", &patternsFound, &calculatedHash, &patternsAction[itemNum]))
        }

        drop(db);

        crate::utils::show::print("Learner", &format!("Training finished! Patterns found: {}; Pair {}; Start date: {}; End date: {}; Pattern threshold: {}; Previous values feed: {}; Forwad values feed: {}", &patternsFound, &pair, &startDate, &endDate, &patternThreshold, &lookBack, &lookForwad))
    }
}