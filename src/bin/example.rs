//! Pool Synchronization Program
//!
//! This program synchronizes pools from a specified blockchain using the PoolSync library.
//! It demonstrates how to set up a provider, configure pool synchronization, and execute the sync process.

use alloy::network::EthereumWallet;
use alloy::providers::ProviderBuilder;
use alloy_node_bindings::anvil::Anvil;
use anyhow::Result;
use futures::stream::{self, StreamExt};
use pool_sync::filter::filter_top_volume;
use pool_sync::{Chain, Pool, PoolInfo, PoolSync, PoolType};
use std::sync::Arc;

use alloy::primitives::{address, Address};
use alloy::signers::local::PrivateKeySigner;
/// The main entry point for the pool synchronization program.
///
/// This function performs the following steps:
/// 1. Loads environment variables
/// 2. Constructs an Alloy provider for the specified chain
/// 3. Configures and builds a PoolSync instance
/// 4. Initiates the pool synchronization process
/// 5. Prints the number of synchronized pools
///
/// # Errors
///
/// This function will return an error if:
/// - The required environment variables are not set
/// - There's an issue constructing the provider or PoolSync instance
/// - The synchronization process fails
use alloy::sol_types::{sol, SolInterface};

sol! {
    #[derive(Debug)]
    #[sol(rpc, bytecode = "60808060405234601557610a94908161001a8239f35b5f80fdfe60806040526004361015610011575f80fd5b5f3560e01c8063980cf65b1461044b5763cc90d2cd1461002f575f80fd5b346103385760203660031901126103385760043567ffffffffffffffff8111610338573660238201121561033857806004013561006b816108ac565b91610079604051938461088a565b8183526024602084019260051b8201019036821161033857602401915b81831061042b578351846100a9826108ac565b916100b7604051938461088a565b8083526100c6601f19916108ac565b015f5b81811061041457505080516100dd816108ac565b906100eb604051928361088a565b8082526100fa601f19916108ac565b015f5b8181106104035750505f5b825181101561033c576001600160a01b0361012382856109c2565b5160405163980cf65b60e01b815291166004820152905f82602481305afa805f9161023b575b600193506102265750815f60033d11610216575b6308c379a0146101e3575b610173575b01610108565b3d156101de573d61018381610929565b90610191604051928361088a565b81525f60203d92013e5b60406101a98151918261088a565b600d81526c2ab735b737bbb71032b93937b960991b60208201526101cd82856109c2565b526101d881846109c2565b5061016d565b61019b565b6101eb6109ea565b806101f7575b50610168565b5f915061020483866109c2565b5261020f82856109c2565b50866101f1565b5060045f803e5f5160e01c61015d565b61023082876109c2565b526101d881866109c2565b90503d805f853e61024c818561088a565b8301926020818503126103385780519067ffffffffffffffff821161033857019261010084820312610338576040519061028582610859565b61028e85610901565b825261029c60208601610901565b6020830152604085015167ffffffffffffffff811161033857816102c1918701610945565b604083015260608501519067ffffffffffffffff821161033857856102ef60e09261032d9460019901610945565b6060850152610300608082016109b4565b608085015261031160a082016109b4565b60a085015261032260c08201610915565b60c085015201610915565b60e082015290610149565b5f80fd5b8184604051918291604083016040845281518091526060840190602060608260051b8701019301915f905b8282106103d257505050508281036020840152815180825260208201916020808360051b8301019401925f915b8383106103a15786860387f35b9193955091936020806103c0600193601f1986820301875289516107ab565b97019301930190928695949293610394565b91936001919395965060206103f28192605f198b820301865288516107cf565b960192019201869594939192610367565b8060606020809386010152016100fd565b60209061041f6108c4565b828287010152016100c9565b82356001600160a01b038116810361033857815260209283019201610096565b34610338576020366003190112610338576004356001600160a01b03811690819003610338576104796108c4565b90604051630dfe168160e01b8152602081600481855afa90811561064e575f91610771575b506001600160a01b0316825260405163d21220a760e01b815290602082600481845afa91821561064e575f92610730575b506001600160a01b0390911660208301908152604051630240bc6b60e21b81529091606090829060049082905afa90811561064e575f905f926106cf575b506001600160701b0391821660e08501521660c083015281516040516395d89b4160e01b81526001600160a01b03909116905f81600481855afa91821561064e576004926020925f916106b5575b5060408601526040519283809263313ce56760e01b82525afa801561064e575f9061067b575b60ff16608084015250516040516395d89b4160e01b81526001600160a01b0390911691905f81600481865afa92831561064e576004936020925f91610659575b50606084015260405163313ce56760e01b815293849182905afa91821561064e575f9261060e575b5060ff61060a921660a08201526040519182916020835260208301906107cf565b0390f35b91506020823d602011610646575b816106296020938361088a565b810103126103385760ff61063f61060a936109b4565b92506105e9565b3d915061061c565b6040513d5f823e3d90fd5b61067591503d805f833e61066d818361088a565b81019061098b565b856105c1565b506020813d6020116106ad575b816106956020938361088a565b81010312610338576106a860ff916109b4565b610581565b3d9150610688565b6106c991503d805f833e61066d818361088a565b8661055b565b9150506060813d606011610728575b816106eb6060938361088a565b81010312610338576106fc81610915565b90604061070b60208301610915565b91015163ffffffff81160361033857906001600160701b0361050d565b3d91506106de565b9091506020813d602011610769575b8161074c6020938361088a565b81010312610338576060610761600492610901565b9291506104cf565b3d915061073f565b90506020813d6020116107a3575b8161078c6020938361088a565b810103126103385761079d90610901565b8361049e565b3d915061077f565b805180835260209291819084018484015e5f828201840152601f01601f1916010190565b9060018060a01b03825116815260018060a01b03602083015116602082015260e06001600160701b0381610829610817604087015161010060408801526101008701906107ab565b606087015186820360608801526107ab565b9460ff608082015116608086015260ff60a08201511660a08601528260c08201511660c086015201511691015290565b610100810190811067ffffffffffffffff82111761087657604052565b634e487b7160e01b5f52604160045260245ffd5b90601f8019910116810190811067ffffffffffffffff82111761087657604052565b67ffffffffffffffff81116108765760051b60200190565b604051906108d182610859565b5f60e083828152826020820152606060408201526060808201528260808201528260a08201528260c08201520152565b51906001600160a01b038216820361033857565b51906001600160701b038216820361033857565b67ffffffffffffffff811161087657601f01601f191660200190565b81601f820112156103385780519061095c82610929565b9261096a604051948561088a565b8284526020838301011161033857815f9260208093018386015e8301015290565b9060208282031261033857815167ffffffffffffffff8111610338576109b19201610945565b90565b519060ff8216820361033857565b80518210156109d65760209160051b010190565b634e487b7160e01b5f52603260045260245ffd5b5f60443d106109b1576040513d600319016004823e8051913d602484011167ffffffffffffffff841117610a58578282019283519167ffffffffffffffff8311610a50573d84016003190185840160200111610a5057506109b19291016020019061088a565b949350505050565b9291505056fea26469706673582212206e944f316740e0e1606fd9b5af3f8f2b067b2e3813a8b54aa0e8369487fbee6364736f6c634300081a0033")]
    UniswapV2DataSync,
    "src/abi/UniswapV2DataSync.json"

}

#[tokio::main]
async fn main() -> Result<()> {
    // Load environment variables from a .env file if present
    dotenv::dotenv().ok();
    let url = std::env::var("ETH")?;
    let anvil = Anvil::new().fork(url).try_spawn()?;
    let signer: PrivateKeySigner = anvil.keys()[0].clone().into();
    let wallet = EthereumWallet::from(signer);

    let provider = Arc::new(
        ProviderBuilder::new()
            .network::<alloy::network::AnyNetwork>()
            .with_recommended_fillers()
            .wallet(wallet)
            .on_http(anvil.endpoint_url()),
    );

    // Configure and build the PoolSync instance
    let pool_sync = PoolSync::builder()
        .add_pool(PoolType::UniswapV2) // Add all the pools you would like to sync
        .chain(Chain::Ethereum) // Specify the chain
        .rate_limit(20) // Specify the rate limit
        .build()?;

    // Initiate the sync process
    let pools = pool_sync.sync_pools(provider.clone()).await?;

    println!("Number of synchronized pools: {}", pools.len());

    /*
    // print out common pool information
    for pool in &pools {
        println!("Pool Address {:?}, Token 0: {:?}, Token 1: {:?}", pool.address(), pool.token0(), pool.token1());
    }

    // extract all pools with top volume tokens
    let pools_over_top_volume_tokens = filter_top_volume(pools, 10).await?;
    */

    // TESTING
    // -----

    // deploy and create contract instance
    let contract = UniswapV2DataSync::deploy(&provider).await?;
    println!("Deployed contract at address: {}", contract.address());

    let addresses: Vec<Address> = pools.iter().take(1000).map(|pool| pool.address()).collect();

    for chunk in addresses.chunks(100) {
        let handle = tokio::spawn( async |chunck, contract| {
            let chunk = chunk.to_vec();
            let res = contract.syncPoolData(chunk).call().await;
            match res {
                Ok(_) => println!("{:#?}", res),
                Err(e) => println!("{:#?}", e),
            }
        });


    }

    Ok(())
}
