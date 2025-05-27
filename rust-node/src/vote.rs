use num_bigint::BigUint;
use serde::{Deserialize, Serialize};

use crate::util::pad_or_trim;

const VOTE_SIZE: usize = 96;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Vote {
    pub index: u64,
    pub request: BigUint,
    pub block_hash: [u8; 32],
}

impl Vote {
    pub fn serialize(&self) -> [u8; VOTE_SIZE] {
        let mut buf = [0u8; VOTE_SIZE];
        let index_bytes = BigUint::from(self.index).to_bytes_be();
        buf[0..32].copy_from_slice(&pad_or_trim(&index_bytes, 32));
        let req_bytes = self.request.to_bytes_be();
        buf[32..64].copy_from_slice(&pad_or_trim(&req_bytes, 32));
        buf[64..96].copy_from_slice(&self.block_hash);
        buf
    }
}
