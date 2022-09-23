use std::{net::Ipv4Addr, path::PathBuf};

use clap::Parser;
use wasi_faas_runtime::Runtime;
use wasi_faas_server::Server;

#[derive(Debug, Parser)]
#[clap(name = "wasi-faas")]
#[clap(about = "An experiment server running WebAssembly System Interface functions", long_about = None)]
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

    Server::builder()
        .with_runtime(runtime)
        .build()
        .serve((args.address, args.port))
        .await?;

    Ok(())
}
