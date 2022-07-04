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

pub mod heart {
    use std::{hash::Hasher, process::exit};
    use wyhash::WyHash;

    #[derive(Debug)]
    pub enum Actions {
        Buy,
        Sell,
        Hold,
    }

    pub fn backtest(
        db: &mut crate::io::db::Database,
        startDate: String,
        endDate: String,
        pair: String,
        initialBalance: f64,
        tradeAmount: f64,
        stopLoss: f64,
        takeProfit: f64,
    ) {
        let data = crate::data::fetcher::retrieve(
            crate::DateTime::<crate::Utc>::from_utc(
                crate::NaiveDate::parse_from_str(&startDate, "%Y-%m-%d")
                    .unwrap()
                    .and_hms(0, 0, 0),
                crate::Utc,
            ),
            crate::DateTime::<crate::Utc>::from_utc(
                crate::NaiveDate::parse_from_str(&endDate, "%Y-%m-%d")
                    .unwrap()
                    .and_hms(23, 59, 59),
                crate::Utc,
            ),
            &pair,
        );

        let storedConfig = db.getConfig();

        let lookBack: usize;

        if let Some(_) = &storedConfig.CsT.unwrap() {
        } else {
            db.close();
            crate::utils::show::printError(
                "Backtesting",
                &String::from("No training database was found!"),
            );
            exit(1);
        }
        if let Some(value) = &storedConfig.ClB.unwrap() {
            lookBack = value[7] as usize
        } else {
            db.close();
            crate::utils::show::printError(
                "Backtesting",
                &String::from("Previous values to get is not defined in the database!"),
            );
            exit(1);
        }
        if ((lookBack + 1) >= data.len()) || (lookBack > 128) {
            db.close();
            crate::utils::show::printError(
                "Backtesting",
                &format!(
                    "Invalid previous values value! (Ticks: {}; Previous Values: {})",
                    data.len(),
                    lookBack,
                ),
            );
            exit(1);
        }

        let mut orders: Vec<(usize, f64, u8)> = vec![];

        for dataNum in (lookBack + 1)..data.len() {
            let mut patternValueDeviation: Vec<u8> = vec![];
            for dataNumDevCalc in (dataNum - lookBack)..(dataNum) {
                let valueDeviation = (((((data[dataNumDevCalc].close
                    / data[dataNumDevCalc - 1].close)
                    * 100 as f64)
                    - 100 as f64)
                    .abs())
                .ln())
                .round() as u8;
                patternValueDeviation.push(valueDeviation)
            }

            let mut hash = WyHash::with_seed(0);
            hash.write(&patternValueDeviation);
            let calculatedHash = hash.finish();

            if let Some(value) = db.getPattern(calculatedHash).unwrap() {
                match value[0] {
                    0 => {
                        orders.push((dataNum, data[dataNum].close, 0u8));
                        continue;
                    }
                    1 => {
                        orders.push((dataNum, data[dataNum].close, 1u8));
                        continue;
                    }
                    2 => {
                        orders.push((dataNum, data[dataNum].close, 2u8));
                        continue;
                    }
                    _ => {}
                }
            }
            if !orders.is_empty() {
                if (((orders.last().unwrap().1 / data[dataNum].close) * 100.0) >= takeProfit)
                    && (orders.last().unwrap().2 != 2u8)
                {
                    orders.push((dataNum, data[dataNum].close, 2u8));
                } else if (((1.0 / (orders.last().unwrap().1 / data[dataNum].close)) * 100.0)
                    <= stopLoss)
                    && (orders.last().unwrap().2 != 2u8)
                {
                    orders.push((dataNum, data[dataNum].close, 2u8));
                }
            }
        }

        let mut balance: f64 = initialBalance;
        let mut balanceHistory: Vec<(usize, f64)> = vec![];

        for orderNum in 1..orders.len() {
            balance += {
                if orders[orderNum].2 == 0u8 {
                    tradeAmount / orders[orderNum - 1].1
                } else if orders[orderNum].2 == 1u8 {
                    -tradeAmount / orders[orderNum - 1].1
                } else {
                    0.0
                }
            } * (orders[orderNum].1 - orders[orderNum - 1].1);
            balanceHistory.push((orders[orderNum].0, balance))
        }

        let _ = crate::plotter::plot::draw(&data, &orders, &balanceHistory);

        db.close();

        crate::utils::show::print(
            "Backtesting",
            &format!("Finished! Final balance: {}", balance),
        );
    }

    pub fn startLearning(
        db: &mut crate::io::db::Database,
        startDate: String,
        endDate: String,
        pair: String,
        lookBack: usize,
        lookForwad: usize,
        patternThreshold: usize,
    ) {
        let data = crate::data::fetcher::retrieve(
            crate::DateTime::<crate::Utc>::from_utc(
                crate::NaiveDate::parse_from_str(&startDate, "%Y-%m-%d")
                    .unwrap()
                    .and_hms(0, 0, 0),
                crate::Utc,
            ),
            crate::DateTime::<crate::Utc>::from_utc(
                crate::NaiveDate::parse_from_str(&endDate, "%Y-%m-%d")
                    .unwrap()
                    .and_hms(23, 59, 59),
                crate::Utc,
            ),
            &pair,
        );

        let storedConfig = db.getConfig();
        if let Some(value) = storedConfig.ClB.unwrap() {
            if ((value[7] as usize) != lookBack)
                && (lookBack != crate::config::defaults::learn::PREVIOUS_VALUES)
            {
                db.close();
                crate::utils::show::printError("Learner", &format!("Previous values to get is different from the one in the database! (Value: {})", (value[7] as usize)));
                exit(1);
            }
        }
        if let Some(value) = storedConfig.ClF.unwrap() {
            if ((value[7] as usize) != lookForwad)
                && (lookForwad != crate::config::defaults::learn::FORWAD_VALUES)
            {
                db.close();
                crate::utils::show::printError(
                    "Learner",
                    &format!(
                        "Following values to get is different from the one in the database! (Value: {})",
                        (value[7] as usize)
                    ),
                );
                exit(1);
            }
        }
        if let Some(value) = storedConfig.CpT.unwrap() {
            if ((value[7] as usize) != patternThreshold)
                && (patternThreshold != crate::config::defaults::learn::PATTERN_THRESHOLD)
            {
                db.close();
                crate::utils::show::printError("Learner", &format!("Pattern Threshold value is different from the one in the database! (Value: {})", (value[7] as usize)));
                exit(1);
            }
        }
        if ((lookBack + 1) as isize >= (data.len() as isize - lookForwad as isize))
            || (lookBack > 128 || lookForwad > 128)
        {
            db.close();
            crate::utils::show::printError(
                "Learner",
                &format!(
                    "Invalid previous values/following values value! (Ticks: {}; Previous Values: {}; Forwad Values: {})",
                    data.len(),
                    lookBack,
                    lookForwad
                ),
            );
            exit(1);
        }

        let _ = db.writeConfig(lookBack, lookForwad, patternThreshold);

        let mut patternsFound: usize = 0;

        let mut patterns: Vec<usize> = vec![];
        let mut patternsAction: Vec<Actions> = vec![];

        for dataNum in (lookBack + 1)..(data.len() - lookForwad) {
            let mut sumForwardValues: f64 = 0.0;
            for dataNumForward in dataNum..(lookForwad + dataNum) {
                sumForwardValues += data[dataNumForward].close
            }
            let averageForwardValues = sumForwardValues / (lookForwad as f64);

            let priceDeviation =
                ((averageForwardValues / data[dataNum].close) * 100 as f64) - 100 as f64;

            if priceDeviation.abs() >= patternThreshold as f64 {
                patterns.push(dataNum);
                if priceDeviation > 1 as f64 {
                    patternsAction.push(Actions::Buy)
                } else if priceDeviation < -1 as f64 {
                    patternsAction.push(Actions::Sell)
                } else {
                    patternsAction.push(Actions::Hold)
                }
            }
        }

        for dataNum in 0..patterns.len() {
            let mut patternValueDeviation: Vec<u8> = vec![];
            for dataNumDevCalc in (patterns[dataNum] - lookBack)..(patterns[dataNum]) {
                let valueDeviation = (((((data[dataNumDevCalc].close
                    / data[dataNumDevCalc - 1].close)
                    * 100 as f64)
                    - 100 as f64)
                    .abs())
                .ln())
                .round() as u8;
                patternValueDeviation.push(valueDeviation)
            }

            let mut hash = WyHash::with_seed(0);
            hash.write(&patternValueDeviation);
            let calculatedHash = hash.finish();

            let _ = db.writePattern(calculatedHash, &patternsAction[dataNum]);

            patternsFound += 1;
            crate::utils::show::print(
                "Learner",
                &format!(
                    "#{} pattern found! Hash: {}; Signal: {:?}",
                    &patternsFound, &calculatedHash, &patternsAction[dataNum]
                ),
            )
        }

        db.close();

        crate::utils::show::print("Learner", &format!("Training finished! Patterns found: {}; Pair {}; Start date: {}; End date: {}; Pattern threshold: {}; Previous values feed: {}; Forwad values feed: {}", &patternsFound, &pair, &startDate, &endDate, &patternThreshold, &lookBack, &lookForwad))
    }
}
