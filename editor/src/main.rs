use process::MuxContext;
use std::ffi::CString;
use clap::Parser;
use anyhow::Context;

#[derive(Parser)]
struct Cli {
    url: String
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let mux_ctx = MuxContext::from_url_cstr(&CString::new(cli.url).unwrap()).context("error creating MuxContext");
    
    Ok(())
}
