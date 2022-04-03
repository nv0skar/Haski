// Haski - Oscar
// The use of this is restricted to only the authors

pub mod file {
    pub mod hashPatterns {        
        pub struct ConfigRetrieve {
            pub ClB: Result<Option<sled::IVec>, sled::Error>,
            pub ClF: Result<Option<sled::IVec>, sled::Error>,
            pub CpT: Result<Option<sled::IVec>, sled::Error>,
        }

        pub fn openDB() -> Result<crate::Db, crate::Error> { crate::open(crate::config::defaults::db::DBPATH) }

        pub fn writePattern(db: &crate::Db, hash: u64, action: &crate::trader::heart::Actions) -> crate::Result<()> {
            let actionBytes: [u8; 1];
            match action {
                crate::trader::heart::Actions::Buy => actionBytes = (0x00 as u8).to_be_bytes(),
                crate::trader::heart::Actions::Sell => actionBytes = (0x01 as u8).to_be_bytes(),
                crate::trader::heart::Actions::Hold => actionBytes = (0x02 as u8).to_be_bytes(),
            }
            let _ = db.insert(hash.to_string(), &actionBytes)?;
            Ok(())
        }
    
        pub fn getConfig(db: &crate::Db) -> ConfigRetrieve {
            ConfigRetrieve { 
                ClB: (db.get("ClB")), ClF: (db.get("ClF")), CpT: (db.get("CpT"))
            }
        }
    
        pub fn writeConfig(db: &crate::Db, pair: &String, lookBack: usize, lookForward: usize, patternThreshold: usize) -> crate::Result<()> {
            let toInsert = crate::HashMap::from([
                ("ClB", lookBack.to_be_bytes()),
                ("ClF", lookForward.to_be_bytes()),
                ("CpT", patternThreshold.to_be_bytes()),
            ]);
            let _ = db.insert("CpA", pair.as_bytes()); for (key, value) in toInsert { db.insert(key, &value)?; }
            Ok(())
        }
    }
}