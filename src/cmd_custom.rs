use std::process::Command;

use serde::{Deserialize, Serialize};

use crate::{cmd_vars::{CMDVariable, CMDVariablesTrait}, exit::exit_status_one, help::{HelpItemCMD, HelpItemCMDProviding}};

// Custom Commands
#[derive(Serialize, Deserialize, Debug)]
pub struct CustomCMD {
    pub names: Vec<String>,
    description: String,
    commands: Vec<String>,
    variables: Vec<CMDVariable>
}

impl CustomCMD {
    pub fn run(&self, args: &Vec<String>) {
        self.variables.exit_if_vars_do_not_match(&args);
        let mut cmd_string = self.commands.join(" && ");
        for (i, var) in self.variables.iter().enumerate() {
            cmd_string = cmd_string.replace(&var.target, &args[i+2]);
        }

        match Command::new(env!("SHELL"))
            .arg("-c")
            .arg(cmd_string)
            .output() {
                Ok(ok) => println!("{}", String::from_utf8(ok.stdout).unwrap()),
                Err(error) => exit_status_one(&format!("{}", error))
            }
    }
}

impl HelpItemCMDProviding for CustomCMD {
    fn help_item_cmd(&self) -> HelpItemCMD {
        HelpItemCMD {
            names: self.names.clone(),
            description: self.description.clone(),
            variables: self.variables.as_help_item_cmd_vars()
        }
    }
}
