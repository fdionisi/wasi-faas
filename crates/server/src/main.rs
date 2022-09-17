mod runtime;
mod server;

use std::{net::Ipv4Addr, path::PathBuf};

use clap::Parser;
use runtime::Runtime;

use crate::server::serve;

#[derive(Debug, Parser)]
#[clap(name = "wasi-faas")]
#[clap(about = "Barebone function-as-a-service for WASI", long_about = None)]
struct Args {
    #[clap(long)]
    module_path: PathBuf,
    #[clap(long, default_value = "0.0.0.0")]
    address: Ipv4Addr,
    #[clap(long, default_value = "8888")]
    port: u16,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let mut runtime = Runtime::new();
    runtime.add_module(args.module_path);

    serve((args.address, args.port), runtime).await?;

    Ok(())
}
