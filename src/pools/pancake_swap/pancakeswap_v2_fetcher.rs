use alloy::primitives::{address, Address};
use alloy_sol_types::SolEvent;
use crate::pools::gen::PancakeSwapV2Factory;
use crate::pools::PoolFetcher;
use alloy::primitives::Log;
use crate::pools::PoolType;
use crate::Chain;

pub struct PancakeSwapV2Fetcher;

impl PoolFetcher for PancakeSwapV2Fetcher {
    fn pool_type(&self) -> PoolType {
        PoolType::PancakeSwapV2
    }

    fn factory_address(&self, chain: Chain) -> Address {
        match chain {
            Chain::Ethereum => address!("1097053Fd2ea711dad45caCcc45EfF7548fCB362"),            
            Chain::Base => address!("02a84c1b3BBD7401a5f7fa98a384EBC70bB5749E"),
        }
    }

    fn pair_created_signature(&self) -> &str {
        PancakeSwapV2Factory::PairCreated::SIGNATURE
    }

    fn log_to_address(&self, log: &Log) -> Address {
        let decoded_log = PancakeSwapV2Factory::PairCreated::decode_log(log, false).unwrap();
        decoded_log.data.pair
    }

}
