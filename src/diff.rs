use std::process::Command;

use anyhow::Result;

/// Get the codebase git diff.
pub fn get_codebase_git_diff()  -> Result<String> {
    let process = Command::new("git")
        .arg("--no-pager")
        .arg("diff")
        .arg("--staged")
        .output()?;

    Ok(String::from_utf8_lossy(&process.stdout).to_string())
}