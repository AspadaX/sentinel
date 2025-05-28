use std::{fmt::Display, str::FromStr};

use anyhow::Error;
use secretary::traits::SystemPrompt;
use serde::{Deserialize, Serialize};

use crate::display_control::{display_message, display_tree_message, format_message, Level};

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

impl CodeLogicCheck {
    fn get_failed_checks_number(&self) -> usize {
        self.implementation_defects.len()
    }
    
    fn get_reasoning(&self) -> &str {
        &self.reasoning
    }
    
    fn get_onverview(&self) -> &str {
        &self.overview
    }
    
    pub fn display_code_logic_check_result(&self) {
        let root_indent_level: usize = 0;
        
        display_tree_message(root_indent_level, &format!("Overview: {}", self.overview));
        
        if self.get_failed_checks_number() != 0 {
            display_tree_message(
                root_indent_level + 1, 
                &format_message(Level::Warn, format!("Number of failures: {}", self.get_failed_checks_number()))
            );
            
            for defect in self.implementation_defects.iter() {
                display_tree_message(
                    root_indent_level + 2, 
                    &format_message(Level::Warn, format!("[DEFECT]: {}", defect.diagnosis))
                );
                
                display_tree_message(
                    root_indent_level + 3, 
                    &format_message(Level::Warn, format!("[SUGGESTION]: {}", defect.suggested_change))
                );
            }
            
            return;
        }
        
        display_tree_message(root_indent_level, &format!("✅ All checks are passed!"));
    }
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

impl FromStr for CodeLogicCheck {
    type Err = Error;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(serde_json::from_str(s)?)
    }
}

impl TryInto<CodeLogicCheck> for String {
    type Error = Error;
    
    fn try_into(self) -> Result<CodeLogicCheck, Self::Error> {
        CodeLogicCheck::from_str(&self)
    }
}

impl SystemPrompt for CodeLogicCheck {
    fn get_system_prompt(&self) -> String {
        format!(
            "You are asked to check the alginment between the git diff output provided and the requirement description.
            You should document these issues in the provide json fromat. However, if there are no issues, you should leave 
            `implementation_defacts` to empty array. if no relevant codes showed up in the git diff output, it is an issue.",
        )
    }
}
