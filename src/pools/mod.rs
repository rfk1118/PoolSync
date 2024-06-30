//! Core definitions for pool synchronization
//!
//! This module defines the core structures and traits used in the pool synchronization system.
//! It includes enumerations for supported pool types, a unified `Pool` enum, and a trait for
//! fetching and decoding pool creation events.

use crate::chain::Chain;
use alloy::primitives::{Address, Log};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::fmt;
use sushiswap::SushiSwapPool;
use uniswap_v2::UniswapV2Pool;
use uniswap_v3::UniswapV3Pool;

// Re-exports
pub use sushiswap::SushiSwapFetcher;
pub use uniswap_v2::UniswapV2Fetcher;
pub use uniswap_v3::UniswapV3Fetcher;

// Pool modules
mod sushiswap;
mod uniswap_v2;
mod uniswap_v3;

/// Enumerates the supported pool types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PoolType {
    /// Uniswap V2 pool type
    UniswapV2,
    /// Uniswap V3 pool type
    UniswapV3,
    /// SushiSwap pool type
    SushiSwap,
}

impl fmt::Display for PoolType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// Represents a populated pool from any of the supported protocols
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Pool {
    /// A Uniswap V2 pool
    UniswapV2(UniswapV2Pool),
    /// A Uniswap V3 pool
    UniswapV3(UniswapV3Pool),
    /// A SushiSwap pool
    SushiSwap(SushiSwapPool),
}

/// Defines common functionality for fetching and decoding pool creation events
///
/// This trait provides a unified interface for different pool types to implement
/// their specific logic for identifying and parsing pool creation events.
#[async_trait]
pub trait PoolFetcher: Send + Sync {
    /// Returns the type of pool this fetcher is responsible for
    fn pool_type(&self) -> PoolType;

    /// Returns the factory address for the given chain
    fn factory_address(&self, chain: Chain) -> Address;

    /// Returns the event signature for pool creation
    fn pair_created_signature(&self) -> &str;

    /// Attempts to create a `Pool` instance from a log entry
    ///
    /// # Arguments
    ///
    /// * `log` - The log entry containing pool creation data
    ///
    /// # Returns
    ///
    /// An `Option<Pool>` which is `Some(Pool)` if the log was successfully parsed,
    /// or `None` if the log did not represent a valid pool creation event.
    async fn from_log(&self, log: &Log) -> Option<Pool>;
}
