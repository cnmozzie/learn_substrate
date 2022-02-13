use crate::cli::Cli;
use structopt::StructOpt;

/// Parse and run command line arguments
pub fn run() -> sc_cli::Result<()> {
    let cli = Cli::from_args();
    println!("{:#?}", cli);

    Ok(())
}

