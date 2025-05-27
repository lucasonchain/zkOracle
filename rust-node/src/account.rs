use num_bigint::BigUint;
use serde::{Deserialize, Serialize};

use crate::util::pad_or_trim;

const ACCOUNT_SIZE: usize = 128;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Account {
    pub index: BigUint,
    pub public_key: (BigUint, BigUint),
    pub balance: BigUint,
}

impl Account {
    pub fn serialize(&self) -> [u8; ACCOUNT_SIZE] {
        let mut buf = [0u8; ACCOUNT_SIZE];
        let index_bytes = self.index.to_bytes_be();
        buf[0..32].copy_from_slice(&pad_or_trim(&index_bytes, 32));
        let pkx = self.public_key.0.to_bytes_be();
        buf[32..64].copy_from_slice(&pad_or_trim(&pkx, 32));
        let pky = self.public_key.1.to_bytes_be();
        buf[64..96].copy_from_slice(&pad_or_trim(&pky, 32));
        let bal = self.balance.to_bytes_be();
        buf[96..128].copy_from_slice(&pad_or_trim(&bal, 32));
        buf
    }

    pub fn deserialize(bytes: &[u8]) -> Self {
        assert!(bytes.len() >= ACCOUNT_SIZE);
        let index = BigUint::from_bytes_be(&bytes[0..32]);
        let pkx = BigUint::from_bytes_be(&bytes[32..64]);
        let pky = BigUint::from_bytes_be(&bytes[64..96]);
        let balance = BigUint::from_bytes_be(&bytes[96..128]);
        Account { index, public_key: (pkx, pky), balance }
    }
}
