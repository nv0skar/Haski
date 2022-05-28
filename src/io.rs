// Haski
// Copyright (C) 2022 ItsTheGuy
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

pub mod file {
    pub mod hashPatterns {
        use sled::{open, Db, Result};
        pub struct ConfigRetrieve {
            pub CsT: Result<Option<sled::IVec>>,
            pub ClB: Result<Option<sled::IVec>>,
            pub ClF: Result<Option<sled::IVec>>,
            pub CpT: Result<Option<sled::IVec>>,
        }

        pub fn openDB(db: String) -> Result<Db> {
            open(db)
        }

        pub fn getPattern(db: &Db, hash: u64) -> Result<Option<sled::IVec>> {
            db.get(hash.to_string())
        }

        pub fn writePattern(
            db: &Db,
            hash: u64,
            action: &crate::trader::heart::Actions,
        ) -> Result<()> {
            let actionBytes: [u8; 1];
            match action {
                crate::trader::heart::Actions::Buy => actionBytes = (0x00 as u8).to_be_bytes(),
                crate::trader::heart::Actions::Sell => actionBytes = (0x01 as u8).to_be_bytes(),
                crate::trader::heart::Actions::Hold => actionBytes = (0x02 as u8).to_be_bytes(),
            }
            let _ = db.insert(hash.to_string(), &actionBytes)?;
            Ok(())
        }

        pub fn getConfig(db: &Db) -> ConfigRetrieve {
            ConfigRetrieve {
                CsT: (db.get("CsT")),
                ClB: (db.get("ClB")),
                ClF: (db.get("ClF")),
                CpT: (db.get("CpT")),
            }
        }

        pub fn writeConfig(
            db: &Db,
            lookBack: usize,
            lookForward: usize,
            patternThreshold: usize,
        ) -> Result<()> {
            let toInsert = crate::HashMap::from([
                ("ClB", lookBack.to_be_bytes()),
                ("ClF", lookForward.to_be_bytes()),
                ("CpT", patternThreshold.to_be_bytes()),
            ]);
            let _ = db.insert("CsT", &0x00_i32.to_be_bytes());
            for (key, value) in toInsert {
                db.insert(key, &value)?;
            }
            Ok(())
        }
    }
}
