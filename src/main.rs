mod arguments;
mod diff;
mod objective;
mod utilities;
mod environment_variables;
mod llm;
mod display_control;

use anyhow::{Error, Result};
use arguments::{Arguments, Commands};
use clap::{Parser, crate_authors, crate_description, crate_name, crate_version};
use display_control::{display_message, Level};
use environment_variables::EnvironmentVariables;
use llm::CodeLogicCheck;
use objective::Objective;
use secretary::openai::OpenAILLM;
use utilities::process_run_arguments_objective_filepath;

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
            let result: CodeLogicCheck = process_run_arguments_objective_filepath(
                llm, 
                Objective::from_file_default(subcommand.objective_filepath.as_deref())?
            )?;
            
            result.display_code_logic_check_result();
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