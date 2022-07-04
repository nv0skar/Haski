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

pub mod db {
    use sled::{open, Db, Result};

    #[derive(Default)]
    pub struct Database {
        db: Option<Db>,
    }

    pub struct Config {
        pub CsT: Result<Option<sled::IVec>>,
        pub ClB: Result<Option<sled::IVec>>,
        pub ClF: Result<Option<sled::IVec>>,
        pub CpT: Result<Option<sled::IVec>>,
    }

    impl Database {
        pub fn open(&mut self, db: String) {
            self.db = Some(open(db).unwrap())
        }

        pub fn getPattern(&mut self, hash: u64) -> Result<Option<sled::IVec>> {
            self.db.as_ref().unwrap().get(hash.to_string())
        }

        pub fn writePattern(
            &mut self,
            hash: u64,
            action: &crate::trader::heart::Actions,
        ) -> Result<()> {
            let actionBytes: [u8; 1];
            match action {
                crate::trader::heart::Actions::Buy => actionBytes = (0x00 as u8).to_be_bytes(),
                crate::trader::heart::Actions::Sell => actionBytes = (0x01 as u8).to_be_bytes(),
                crate::trader::heart::Actions::Hold => actionBytes = (0x02 as u8).to_be_bytes(),
            }
            let _ = self
                .db
                .as_ref()
                .unwrap()
                .insert(hash.to_string(), &actionBytes)?;
            Ok(())
        }

        pub fn getConfig(&mut self) -> Config {
            Config {
                CsT: (self.db.as_ref().unwrap().get("CsT")),
                ClB: (self.db.as_ref().unwrap().get("ClB")),
                ClF: (self.db.as_ref().unwrap().get("ClF")),
                CpT: (self.db.as_ref().unwrap().get("CpT")),
            }
        }

        pub fn writeConfig(
            &mut self,
            lookBack: usize,
            lookForward: usize,
            patternThreshold: usize,
        ) -> Result<()> {
            let toInsert = crate::HashMap::from([
                ("ClB", lookBack.to_be_bytes()),
                ("ClF", lookForward.to_be_bytes()),
                ("CpT", patternThreshold.to_be_bytes()),
            ]);
            let _ = self
                .db
                .as_ref()
                .unwrap()
                .insert("CsT", &0x00_i32.to_be_bytes());
            for (key, value) in toInsert {
                self.db.as_ref().unwrap().insert(key, &value)?;
            }
            Ok(())
        }

        pub fn close(&mut self) {
            let _ = self.db.as_ref().unwrap().flush();
            drop(self.db.as_ref());
        }
    }
}
