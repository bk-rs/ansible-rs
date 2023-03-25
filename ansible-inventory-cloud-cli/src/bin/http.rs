/*
Ref ansible_inventory_cloud.sh
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
