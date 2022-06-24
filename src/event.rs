use anyhow::Result;
use blake2::digest::{Update, VariableOutput};
use blake2::Blake2bVar;
use serde::{Deserialize, Serialize};
use std::str;

const SIZE: usize = 4;

pub struct Event {
    pub project_id: i64,
    pub date: i64,
    pub hash: String,
}

impl Event {
    pub fn new(project_id: i64, date: i64) -> Result<Self> {
        let hash = Hash { project_id, date }.get_hash()?;

        Ok(Event {
            project_id,
            date,
            hash,
        })
    }
}

#[derive(Serialize, Deserialize)]
struct Hash {
    pub project_id: i64,
    pub date: i64,
}

impl Hash {
    fn get_hash(&self) -> Result<String> {
        let data = serde_pickle::to_vec(self, Default::default())?;

        let mut hasher = Blake2bVar::new(SIZE)?;
        hasher.update(&data);

        let mut hash = [0u8; SIZE];
        hasher.finalize_variable(&mut hash)?;

        Ok(hex::encode(&hash))
    }
}
