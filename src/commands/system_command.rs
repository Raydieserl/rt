use super::{command_trait::CommandTrait, command_variables::CommandVariable};

#[derive(Debug, PartialEq)]
pub enum SystemCommandType {
    Help,
    Version,
    Shell,
    Export,
    Import,
    Remove,
    Add
}

// Custom Commands
#[derive(Debug)]
pub struct SystemCommand {
    pub command_type: SystemCommandType,
    triggers: Vec<String>,
    description: Option<String>,
    variables: Option<Vec<CommandVariable>>,
    groups: Option<Vec<String>>
}

impl SystemCommand {
    pub fn new(
        command_type: SystemCommandType,
        triggers: Vec<String>,
        description: Option<String>,
        variables: Option<Vec<CommandVariable>>,
        groups: Option<Vec<String>>
    ) -> SystemCommand {
        SystemCommand { command_type, triggers, description, variables, groups }
    }
}

impl CommandTrait for SystemCommand {
    fn triggers(&self) -> &Vec<String> {
        &self.triggers
    }
    fn description(&self) -> Option<&String> {
        self.description.as_ref()
    }
    fn variables(&self) -> Option<&Vec<CommandVariable>> {
        self.variables.as_ref()
    }
    fn groups(&self) -> Option<&Vec<String>> {
        self.groups.as_ref()
    }
}