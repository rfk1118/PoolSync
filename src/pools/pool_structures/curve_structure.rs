use alloy::primitives::Address;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CurvePool {
    pub address: Address,
    pub tokens: Vec<Address>,
    pub token0: Address,
    pub token1: Address,
    pub token2: Option<Address>,
    pub token0_name: String,
    pub token1_name: String,
    pub token0_decimals: u8,
    pub token1_decimals: u8,
    pub token2_name: Option<String>,
    pub token2_decimals: Option<u8>,
}