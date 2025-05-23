use std::fmt::Display;

use anyhow::Result;
use rsfile::{read_binary, write_text_once};
use serde::{Deserialize, Serialize};

/// Represents a set of objectives with a main objective and sub-objectives.
///
/// This structure holds a primary objective along with a list of related secondary objectives.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Objective {
    /// The primary or main objective
    main_objective: String,
    /// A list of secondary or supporting objectives
    objectives: Vec<String>
}

impl Objective {
    /// Load the `Objective` from an objective.json file
    pub fn from_file(filepath: &str) -> Result<Self> {
        Ok(
            serde_json::from_slice(&read_binary(filepath))?
        )
    }
    
    /// Save the object into a json
    pub fn save(&self, filepath: &str) -> Result<()> {
        let content: String = serde_json::to_string_pretty(&self)?;
        write_text_once(filepath, &content);
        Ok(())
    }
}

impl Default for Objective {
    fn default() -> Self {
        Self {
            main_objective: "The main purpose of this ticket/task".to_string(), 
            objectives: vec![
                "Concrete steps/goals to fullfill in this ticket/task".to_string()
            ]
        }
    }
}

impl Display for Objective {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            &serde_json::to_string_pretty(&self).unwrap()
        )
    }
}
