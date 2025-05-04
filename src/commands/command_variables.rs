use serde::{Serialize, Deserialize};

use super::exit::exit_status_one;


type CommandVariables = Vec<CommandVariable>;
pub trait CommandVariablesTrait {
    fn exit_if_vars_do_not_match(&self, variables: &Vec<&String>);
}

impl CommandVariablesTrait for CommandVariables {
    fn exit_if_vars_do_not_match(&self, variables: &Vec<&String>) {
        if self.len() > 0  && variables.len() < self.len() {
            let targets: Vec<String> = self.iter().map(|e| e.target.clone()).collect();
            exit_status_one(&format!("Missing arguments for command: {}", targets.join(", ")));
        }
    }
}

// Command Var
#[derive(Serialize, Deserialize, Debug)]
pub struct CommandVariable {
    pub description: String,
    pub target: String
}