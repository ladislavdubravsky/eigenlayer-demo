//! Hello world of Ethereum node interaction.

use alloy::providers::{Provider, ProviderBuilder};
use eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let rpc_url = "https://eth.merkle.io".parse()?;
    let provider = ProviderBuilder::new().on_http(rpc_url);

    let chain_id = provider.get_chain_id().await?;
    let block_number = provider.get_block_number().await?;
    println!("Chain id: {}, current block number: {}", chain_id, block_number);

    Ok(())
}
