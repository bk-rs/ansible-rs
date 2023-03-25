pub use ansible_inventory_cloud_cli::*;

use clap::Parser as _;

use self::args::{Args, ArgsSubcommand};

//
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //
    let args = Args::parse();

    //
    match args.subcommand {
        #[allow(unused_variables)]
        ArgsSubcommand::Http(args) => {
            #[cfg(feature = "with_http")]
            {
                match http::run(args).await {
                    Ok(_) => {}
                    Err(err) => {
                        eprintln!("{err}")
                    }
                }
            }
            #[cfg(not(feature = "with_http"))]
            {
                eprintln!("please enable feature 'with_http'")
            }
        }
    }

    Ok(())
}
