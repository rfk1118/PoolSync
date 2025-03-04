//! PoolSync Core Implementation
//!
//! This module contains the core functionality for synchronizing pools across different
//! blockchain networks and protocols. It includes the main `PoolSync` struct and its
//! associated methods for configuring and executing the synchronization process.
//!
use alloy::providers::Provider;
use alloy::providers::ProviderBuilder;
use std::collections::HashMap;
use std::sync::Arc;

use crate::builder::PoolSyncBuilder;
use crate::cache::{read_cache_file, write_cache_file, PoolCache};
use crate::chain::Chain;
use crate::errors::*;
use crate::pools::*;
use crate::rpc::Rpc;

/// The main struct for pool synchronization
pub struct PoolSync {
    /// Map of pool types to their fetcher implementations
    pub fetchers: HashMap<PoolType, Arc<dyn PoolFetcher>>,
    /// The chain to sync on
    pub chain: Chain,
    /// The rate limit of the rpc
    pub rate_limit: u64,
}

impl PoolSync {
    /// Construct a new builder to configure sync parameters
    pub fn builder() -> PoolSyncBuilder {
        PoolSyncBuilder::default()
    }

    /// Synchronizes all added pools for the specified chain
    pub async fn sync_pools(&self) -> Result<(Vec<Pool>, u64), PoolSyncError> {
        // load in the dotenv
        dotenv::dotenv().ok();

        // setup arvhice node provider
        let archive = Arc::new(
            ProviderBuilder::new()
                .network::<alloy::network::AnyNetwork>()
                .on_http(std::env::var("ARCHIVE").unwrap().parse().unwrap()),
        );

        // setup full node provider
        let full = Arc::new(
            ProviderBuilder::new()
                .network::<alloy::network::AnyNetwork>()
                .on_http(std::env::var("FULL").unwrap().parse().unwrap()),
        );

        // create the cache files
        std::fs::create_dir_all("cache").unwrap();

        // create all of the caches
        let mut pool_caches: Vec<PoolCache> = self
            .fetchers
            .keys()
            .map(|pool_type| read_cache_file(pool_type, self.chain).unwrap())
            .collect();

        let mut fully_synced = false;
        let mut last_synced_block = 0;

        while !fully_synced {
            fully_synced = true;
            let end_block = full.get_block_number().await.unwrap();

            for cache in &mut pool_caches {
                let start_block = cache.last_synced_block + 1;
                if start_block <= end_block {
                    fully_synced = false;

                    let fetcher = self.fetchers[&cache.pool_type].clone();

                    // fetch all of the pool addresses
                    let pool_addrs = Rpc::fetch_pool_addrs(
                        start_block,
                        end_block,
                        archive.clone(),
                        fetcher.clone(),
                        self.chain,
                        self.rate_limit,
                    )
                    .await
                    .expect(
                        "Failed to fetch pool addresses. Exiting due to having inconclusive state",
                    );

                    // populate all of the pool data
                    let mut new_pools = Rpc::populate_pools(
                        pool_addrs,
                        full.clone(),
                        cache.pool_type,
                        fetcher.clone(),
                        self.rate_limit,
                        self.chain,
                    )
                    .await
                    .expect("Failed to sync pool data, Exiting due to haveing inconclusive state");


                    // catch up all the old pools
                    Rpc::populate_liquidity(
                        start_block,
                        end_block,
                        &mut cache.pools,
                        archive.clone(),
                        cache.pool_type,
                        self.rate_limit,
                        cache.is_initial_sync,
                    )
                    .await
                    .expect("Failed to populate liquidity information, Exiting due to having inconclusive state");

                    // update the new pools
                    if !new_pools.is_empty() {
                        Rpc::populate_liquidity(
                            start_block,
                            end_block,
                            &mut new_pools,
                            archive.clone(),
                            cache.pool_type,
                            self.rate_limit,
                            true,
                        )
                        .await
                        .expect("Failed to populate liquidity information, Exiting due to having inconclusive state");
                    }


                    // merge old and new
                    cache.pools.extend(new_pools);


                    // update info for cache
                    cache.last_synced_block = end_block;
                    last_synced_block = end_block;
                    cache.is_initial_sync = false;
                }
            }
        }

        // write all of the cache files
        pool_caches
            .iter()
            .for_each(|cache| write_cache_file(cache, self.chain).unwrap());

        // return all the pools
        Ok((
            pool_caches
                .into_iter()
                .flat_map(|cache| cache.pools)
                .collect(),
            last_synced_block,
        ))
    }
}
