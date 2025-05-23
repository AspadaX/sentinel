use anyhow::{Ok, Result};

#[derive(Debug, Clone)]
pub struct EnvironmentVariables {
    api_key: String,
    api_base: String,
    model: String,
}

impl EnvironmentVariables {
    pub fn new() -> Result<Self> {
        Ok(
            Self { 
                api_key: std::env::var("SENTINEL_OPENAI_API_BASE")?, 
                api_base: std::env::var("SENTINEL_OPENAI_API_KEY")?, 
                model: std::env::var("SENTINEL_OPENAI_MODEL")?
            }
        )
    }
    
    pub fn get_api_key(&self) -> &str {
        &self.api_key
    }
    
    pub fn get_api_base(&self) -> &str {
        &self.api_base
    }
    
    pub fn get_model(&self) -> &str {
        &self.model
    }
}