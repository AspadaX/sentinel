use anyhow::Result;
use secretary::{tasks::basic_task::BasicTask, traits::{GenerateJSON, IsLLM, SystemPrompt}};

use crate::{diff::get_codebase_git_diff, llm::CodeLogicCheck, objective::Objective};

/// Process git diff + objective.json to generate a diagnosis
pub fn process_run_arguments_objective_filepath(
    llm: impl GenerateJSON + IsLLM,
    objective_file_data: Objective,
) -> Result<String> {
    let task = BasicTask::new(
        CodeLogicCheck::default(), 
        vec![CodeLogicCheck::default().get_system_prompt()]
    );
    
    // Get the objective.json content as well as the git diff output
    let target = format!(
        "Requrements:\n{}\nCode:\n{}", 
        objective_file_data, get_codebase_git_diff()?
    );
    
    // send to the LLM for generation
    Ok(llm.generate_json(&task, &target)?)
}