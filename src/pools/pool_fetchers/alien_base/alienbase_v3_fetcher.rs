use crate::pools::gen::AlienBaseV3Factory;
use alloy::primitives::{address, Address};
use alloy_sol_types::SolEvent;
use crate::pools::PoolFetcher;
use alloy::primitives::Log;
use crate::pools::PoolType;
use crate::Chain;
use alloy::dyn_abi::DynSolType;

pub struct AlienBaseV3Fetcher;

impl PoolFetcher for AlienBaseV3Fetcher {
    fn pool_type(&self) -> PoolType {
        PoolType::AlienBaseV3
    }

    fn factory_address(&self, chain: Chain) -> Address {
        match chain {
            Chain::Base => address!("B27f110571c96B8271d91ad42D33A391A75E6030"),
            _ => panic!("SushiSwapV3 not supported on this chain")
        }
    }
    
    fn pair_created_signature(&self) -> &str {
        AlienBaseV3Factory::PoolCreated::SIGNATURE
    }

    fn log_to_address(&self, log: &Log) -> Address {
        let decoded_log = AlienBaseV3Factory::PoolCreated::decode_log(log, false).unwrap();
        decoded_log.data.pool
    }

    fn get_pool_repr(&self) -> DynSolType {
        DynSolType::Array(Box::new(DynSolType::Tuple(vec![
            DynSolType::Address,
            DynSolType::Address,
            DynSolType::Uint(8),
            DynSolType::Address,
            DynSolType::Uint(8),
            DynSolType::Uint(128),
            DynSolType::Uint(160),
            DynSolType::Int(24),
            DynSolType::Int(24),
            DynSolType::Uint(24),
            DynSolType::Int(128),
        ])))
    }
}