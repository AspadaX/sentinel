use std::fmt::Display;

use secretary::traits::SystemPrompt;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImplementationDefact {
    diagnosis: String,
    suggested_change: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeLogicCheck {
    reasoning: String,
    overview: String,
    implementation_defects: Vec<ImplementationDefact>
}

impl Display for ImplementationDefact {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            &serde_json::to_string_pretty(&self).unwrap()
        )
    }
}

impl Display for CodeLogicCheck {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            &serde_json::to_string_pretty(&self).unwrap()
        )
    }
}

impl Default for ImplementationDefact {
    fn default() -> Self {
        Self {
            diagnosis: "Your diagnosis to the code logic".to_string(), 
            suggested_change: "What changes that you think will resolve the diagnosis you have provided".to_string()
        }
    }
}

impl Default for CodeLogicCheck {
    fn default() -> Self {
        Self {
            reasoning: "Your reasonings in diagnosing the code".to_string(), 
            overview: "Your overall comment to the code".to_string(), 
            implementation_defects: vec![ImplementationDefact::default()]
        }
    }
}

impl SystemPrompt for CodeLogicCheck {
    fn get_system_prompt(&self) -> String {
        format!(
            "You are asked to check the alginment between the code provided and the requirement description.
            If a piece of code does not align with the requirement, it is an issue. You should document these issues
            in the provide json fromat.",
        )
    }
}
