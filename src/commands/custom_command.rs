use std::process::Command;

use serde::{Deserialize, Serialize};

use super::command_variables::{CommandVariable, CommandVariablesTrait};
use super::exit::exit_status_one;

// Custom Commands
#[derive(Serialize, Deserialize, Debug)]
pub struct CustomCommand {
    pub triggers: Vec<String>,
    pub description: String,
    commands: Vec<String>,
    pub variables: Vec<CommandVariable>
}

impl CustomCommand {
    pub fn run(&self, args: &Vec<String>) {
        self.variables.exit_if_vars_do_not_match(&args);
        let mut command_string = self.commands.join(" && ");
        for (i, var) in self.variables.iter().enumerate() {
            command_string = command_string.replace(&var.target, &args[i+2]);
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
