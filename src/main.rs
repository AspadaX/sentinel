mod arguments;
mod diff;
mod objective;
mod utilities;
mod environment_variables;
mod llm;

use anyhow::{Error, Result};
use arguments::{Arguments, Commands};
use cchain::display_control::{Level, display_message};
use clap::{Parser, crate_authors, crate_description, crate_name, crate_version};
use environment_variables::EnvironmentVariables;
use objective::Objective;
use secretary::openai::OpenAILLM;

fn main() -> Result<(), Error> {
    let arguments = Arguments::parse();
    
    // Instantiate resources
    let environment_variables: EnvironmentVariables = EnvironmentVariables::new()?;
    let llm: OpenAILLM = OpenAILLM::new(
        environment_variables.get_api_base(),
        environment_variables.get_api_key(),
        environment_variables.get_model()
    )?;

    match arguments.commands {
        Commands::Run(subcommand) => {
            let objective_data_content = 
        },
        Commands::Generate(subcommand) => {},
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