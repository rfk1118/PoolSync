
use crate::pools::gen::AlienBaseFactory;
use alloy::primitives::{address, Address};
use alloy_sol_types::SolEvent;
use crate::pools::PoolFetcher;
use alloy::primitives::Log;
use crate::pools::PoolType;
use crate::Chain;

pub struct AlienBaseFetcher;

impl PoolFetcher for AlienBaseFetcher {
    fn pool_type(&self) -> PoolType {
        PoolType::AlienBase
    }

    fn factory_address(&self, chain: Chain) -> Address {
        match chain {
            Chain::Base => address!("0Fd83557b2be93617c9C1C1B6fd549401C74558C"),
            _=> panic!("AlienBase not supported on this chain")
        }
    }

    fn pair_created_signature(&self) -> &str {
        AlienBaseFactory::PoolCreated::SIGNATURE
    }

    fn log_to_address(&self, log: &Log) -> Address {
        let decoded_log = AlienBaseFactory::PoolCreated::decode_log(log, false).unwrap();
        decoded_log.data.pool
        
    }

}