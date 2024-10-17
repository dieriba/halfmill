use serde::Serialize;

pub mod jobs;

#[derive(Debug, Serialize)]
pub struct ScriptOutput {
    output: String,
}

impl ScriptOutput {
    pub fn new(output: String) -> Self {
        Self { output }
    }
}
