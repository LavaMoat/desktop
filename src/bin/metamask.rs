use anyhow::Result;
use clap::Parser;
use log::{error, warn};
use std::net::SocketAddr;
use tokio::sync::oneshot;

use metamask::*;

/// MetaMask crypto wallet.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// Do not launch a window.
    #[clap(short, long)]
    headless: bool,

    /// Bind address for the server.
    #[clap(short, long, default_value = "0.0.0.0:7777")]
    address: SocketAddr,

    /// URL for the window.
    #[clap(short, long, default_value = "http://localhost:7777")]
    url: String,
}

fn print_error(e: anyhow::Error) {
    if let Some(e) = e.downcast_ref::<std::io::Error>() {
        if let std::io::ErrorKind::AddrInUse = e.kind() {
            warn!(
                "Could not start the server because the address is being used!"
            );
            warn!("This happens when a server is already running on a port,");
            warn!("which can happen if metamask is already running.");
            warn!("");
            warn!("To fix this problem stop the service using the port");
            error!("{}", e);
        }
    } else {
        error!("{}", e);
    }
    std::process::exit(1);
}

async fn run() -> Result<()> {
    let args = Cli::parse();

    let addr = args.address;
    let title = "MetaMask";
    let url = args.url;
    let (tx, rx) = oneshot::channel::<Option<SocketAddr>>();

    //info!("Starting server at {}", addr);

    //let mut ctrlc_count = 0;
    ctrlc::set_handler(move || {
        std::process::exit(0);
        // TODO: graceful server shutdown
        /*
        ctrlc_count += 1;
        if ctrlc_count == 2 {
            std::process::exit(0);
        }
        */
    })
    .expect("could not set Ctrl-C signal handler");

    // Web server must be spawned on a separate thread
    // as we need the main thread for the UI window
    let server_handle = std::thread::spawn(move || match server(addr, tx) {
        Ok(_) => {}
        Err(e) => print_error(e),
    });

    match rx.await {
        Ok(_addr) => {
            if !args.headless {
                // Window must be opened on the main thread
                window(url, title)?;
            } else {
                let _ = server_handle.join();
            }
        }
        Err(_) => {} /* Ignore channel closed error */
    }

    // Must loop so that stderr can be flushed otherwise
    // the program can exit before error messages have
    // finished printing
    loop {}
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    std::env::set_var("RUST_LOG", "info");
    pretty_env_logger::init();
    match run().await {
        Ok(_) => {}
        Err(e) => print_error(e),
    }
}
