/*
RUST_BACKTRACE=1 RUST_LOG=trace cargo run -p ansible-inventory-cloud-cli --features with_http \
    --bin ansible-inventory-http-cli -- \
    --list-url 'http://127.0.0.1:8080/ansible_inventory/list' \
    --host-url 'http://127.0.0.1:8080/ansible_inventory/host' \
    --access-token "TOKEN" \
    --list

cargo install ansible-inventory-cloud-cli
ansible-inventory-http-cli \
    --list-url 'https://manager.vpnflash.com/api/v3_node_ansible_inventory/list' \
    --host-url 'https://manager.vpnflash.com/api/v3_node_ansible_inventory/host' \
    --access-token "${V_NODE_MANAGER_V3_NODE_ANSIBLE_INVENTORY_ACCESS_TOKEN}" \
    --list
*/

pub use ansible_inventory_cloud_cli::*;

use clap::Parser as _;

use self::args::HttpArgs;

//
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //
    let args = HttpArgs::parse();

    //
    match http::run(args).await {
        Ok(_) => {}
        Err(err) => {
            eprintln!("{err}")
        }
    }

    Ok(())
}
