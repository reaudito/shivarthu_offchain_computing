#![allow(missing_docs)]
use subxt::backend::{legacy::LegacyRpcMethods, rpc::RpcClient};
use subxt::{OnlineClient, PolkadotConfig};

#[subxt::subxt(runtime_metadata_path = "./artifacts/metadata.scale")]
pub mod polkadot {}

#[tokio::main]
pub async fn subscribe_block(block_number: u32) -> Result<(), Box<dyn std::error::Error>> {
    // First, create a raw RPC client:
    let rpc_client = RpcClient::from_url("ws://127.0.0.1:9944").await?;

    // Use this to construct our RPC methods:
    let rpc = LegacyRpcMethods::<PolkadotConfig>::new(rpc_client.clone());

    // We can use the same client to drive our full Subxt interface too:
    let api = OnlineClient::<PolkadotConfig>::from_rpc_client(rpc_client.clone()).await?;
    // Create a client to use:
    // Get the hash from block number
    let block_hash = rpc.chain_get_block_hash(Some(block_number.into())).await?;

    // Subscribe to all finalized blocks:
    let block = api.blocks().at(block_hash.unwrap()).await?;

    let block_number = block.header().number;
    let block_hash = block.hash();

    println!("Block #{block_number}:");
    println!("  Hash: {block_hash}");
    println!("  Extrinsics:");

    // Log each of the extrinsic with it's associated events:
    let extrinsics = block.extrinsics().await?;
    for ext in extrinsics.iter() {
        let ext = ext?;
        let idx = ext.index();
        let events = ext.events().await?;
        let bytes_hex = format!("0x{}", hex::encode(ext.bytes()));

        // See the API docs for more ways to decode extrinsics:
        let decoded_ext = ext.as_root_extrinsic::<polkadot::Call>();

        println!("    Extrinsic #{idx}:");
        println!("      Bytes: {bytes_hex}");
        println!("      Decoded: {decoded_ext:?}");

        println!("      Events:");
        for evt in events.iter() {
            let evt = evt?;
            let pallet_name = evt.pallet_name();
            let event_name = evt.variant_name();
            let event_values = evt.field_values()?;

            println!("        {pallet_name}_{event_name}");
            println!("          {}", event_values);
        }

        println!("      Signed Extensions:");
        if let Some(signed_extensions) = ext.signed_extensions() {
            for signed_extension in signed_extensions.iter() {
                let signed_extension = signed_extension?;
                let name = signed_extension.name();
                let value = signed_extension.value()?.to_string();
                println!("        {name}: {value}");
            }
        }
    }

    Ok(())
}
