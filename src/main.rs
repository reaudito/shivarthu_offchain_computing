use axum::{extract::Extension, routing::get, Router};
use std::sync::Arc;

use offchain_computing::app_state::AppState;

use offchain_computing::functions::test_functions::get_user;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // build our application with a single route

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    let node = iroh::node::Node::persistent("storage")
        .await?
        .spawn()
        .await
        .unwrap();
    let peer = node.node_id();
    let addrs = node.local_endpoint_addresses().await?;
    println!("node PeerID:     {peer}");
    println!("node listening addresses:");
    for addr in addrs {
        println!("    {}", addr);
    }
    let shared_state = Arc::new(AppState { iroh: node });

    let app = Router::new()
        .route("/", get(get_user))
        .layer(Extension(shared_state));
    axum::serve(listener, app).await.unwrap();
    Ok(())
}
