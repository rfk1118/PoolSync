use alloy::primitives::{address, Address};
use crate::pools::gen::TriCryptoFactory;
use crate::pools::pool_structure::CurvePool;
use alloy_sol_types::SolEvent;
use crate::pools::PoolFetcher;
use alloy::primitives::Log;
use crate::pools::PoolType;
use crate::Pool;
use crate::Chain;

pub struct CurveTriCryptoFetcher;

impl PoolFetcher for CurveTriCryptoFetcher {
    fn pool_type(&self) -> PoolType {
        PoolType::CurveTriCrypto
    }

    fn factory_address(&self, chain: Chain) -> Address {
        match chain {
            Chain::Ethereum => address!("0c0e5f2fF0ff18a3be9b835635039256dC4B4963"),
            Chain::Base => address!("A5961898870943c68037F6848d2D866Ed2016bcB"),
        }
    }


    fn pair_created_signature(&self) -> &str {
        TriCryptoFactory::TricryptoPoolDeployed::SIGNATURE
    }

    fn log_to_address(&self, log: &Log) -> Address {
        let decoded_log = TriCryptoFactory::TricryptoPoolDeployed::decode_log(log, false).unwrap();
        decoded_log.data.pool
    }

}