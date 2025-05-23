mod arguments;
mod diff;
mod objective;
mod utilities;

use anyhow::{Error, Result};
use arguments::{Arguments, Commands};
use cchain::display_control::{Level, display_message};
use clap::{Parser, crate_authors, crate_description, crate_name, crate_version};

fn main() -> Result<(), Error> {
    let arguments = Arguments::parse();

    match arguments.commands {
        Commands::Run(subcommand) => {}
        Commands::Generate(subcommand) => {}
        Commands::Version(_) => {
            display_message(Level::Logging, &format!("{}", crate_name!()));
            display_message(Level::Logging, &format!("version.{}", crate_version!()));
            display_message(Level::Logging, &format!("Authors: {}", crate_authors!()));
            display_message(
                Level::Logging,
                &format!("Description: {}", crate_description!()),
            );
        }
    }

    Ok(())
}