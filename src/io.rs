// Haski - Oscar
// The use of this is restricted to only the authors

use std::collections::HashMap;
use sled::{Result, open, Db, Error};

pub mod file {
    pub fn openDB() -> Result<super::Db, super::Error> { super::open("db") }

    pub fn writePattern(db: &super::Db, hash: u64, action: &crate::trader::heart::Actions) -> super::Result<()> {
        let actionBytes: [u8; 1];
        match action {
            crate::trader::heart::Actions::Buy => actionBytes = (0x00 as u8).to_be_bytes(),
            crate::trader::heart::Actions::Sell => actionBytes = (0x01 as u8).to_be_bytes(),
            crate::trader::heart::Actions::Hold => actionBytes = (0x02 as u8).to_be_bytes(),
        }
        let _ = db.insert(hash.to_string(), &actionBytes)?;
        Ok(())
    }

    pub fn writeConfig(db: &super::Db, lookBack: usize, lookForward: usize, patternThreshold: usize) -> super::Result<()> {
        let toInsert = super::HashMap::from([
            ("c.lB", lookBack.to_be_bytes()),
            ("c.lF", lookForward.to_be_bytes()),
            ("c.pT", patternThreshold.to_be_bytes()),
        ]);
        for (key, value) in toInsert { db.insert(key, &value)?; }
        Ok(())
    }
}