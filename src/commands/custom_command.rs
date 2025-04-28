use std::process::Command;

use serde::{Deserialize, Serialize};

use super::command_trait::CommandTrait;
use super::command_variables::{CommandVariable, CommandVariablesTrait};
use super::exit::exit_status_one;

// Custom Commands
#[derive(Serialize, Deserialize, Debug)]
pub struct CustomCommand {
    triggers: Vec<String>,
    description: Option<String>,
    commands: Vec<String>,
    variables: Option<Vec<CommandVariable>>,
    groups: Option<Vec<String>>
}

impl CustomCommand {
    pub fn new(
        triggers: Vec<String>,
        description: Option<String>,
        commands: Vec<String>,
        variables: Option<Vec<CommandVariable>>,
        groups: Option<Vec<String>>
    ) -> CustomCommand {
        CustomCommand {
            triggers,
            description,
            commands,
            variables,
            groups
        } 
    }

    pub fn run(&self, args: &Vec<String>) {
        let mut command_string = self.commands.join(" && ");

        if let Some(vars) = &self.variables {
            vars.exit_if_vars_do_not_match(&args);
            for (i, var) in vars.iter().enumerate() {
                command_string = command_string.replace(&var.target, &args[i+2]);
            }
        }

        match Command::new(env!("SHELL"))
            .arg("-c")
            .arg(command_string)
            .output() {
                Ok(ok) => println!("{}", String::from_utf8(ok.stdout).unwrap()),
                Err(error) => exit_status_one(&format!("{}", error))
            }
    }
}

impl CommandTrait for CustomCommand {
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