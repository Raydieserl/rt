use std::process::Command;

use serde::{Serialize, Deserialize};

use crate::exit::exit_status_one;
use crate::help::{HelpItem, HelpItemCMD, HelpItemCMDProviding, HelpProviding};
use crate::cmd_vars::{CMDVariable, CMDVariablesTrait};


// Custom Commands
#[derive(Serialize, Deserialize, Debug)]
pub struct CustomCMD {
    names: Vec<String>,
    description: String,
    commands: Vec<String>,
    variables: Vec<CMDVariable>
}

impl CustomCMD {
    fn run(&self, args: &Vec<String>) {
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


// CustomCMDs
pub type CustomCMDs = Vec<CustomCMD>;
pub trait CustomCMDsTrait {
    fn run(&self, args: &Vec<String>);
    fn remove_by_name(&mut self, args: &Vec<String>);
    fn get_item_and_idx(&self, name: &String) -> Option<(&CustomCMD, usize)>;
}

impl CustomCMDsTrait for CustomCMDs {
    fn run(&self, args: &Vec<String>) {
        if let Some((ccmd, _)) = self.get_item_and_idx(&args[1]) {
            ccmd.run(args);
        }
    }

    fn remove_by_name(&mut self, args: &Vec<String>) {
        if let Some((_, i)) = self.get_item_and_idx(&args[2]) {
            self.remove(i);
        }
    }

    fn get_item_and_idx(&self, name: &String) -> Option<(&CustomCMD, usize)> {
        for (i, ccmd) in self.iter().enumerate() {
            if ccmd.names.contains(&name) { 
                return Some((ccmd, i));
            }
        }
        exit_status_one(&format!("Custom command not found: {}", name));
        return None;
    }
}

impl HelpProviding for CustomCMDs {
    fn help_item(&self) -> crate::help::HelpItem {
        HelpItem {
            title: "Custom Commands: ".to_string(),
            commands: self.iter().map(|e|e.help_item_cmd()).collect()
        }
    }
}

/* 
// Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_update_command_with_variables_when_arg_is_variable_then_replace_with_args_value() {
        let cmd = CMD { 
            cmd: "ls".to_string(), 
            args: vec!["-l".to_string(), "<PATH>".to_string()]
        };
        let vars = vec![
            CMDVar {
                target: "<PATH>".to_string(),
                description: "test description".to_string()
            }
        ];
        let args = vec!["rt".to_string(), "testcmd".to_string(), ".".to_string()];
        let new_cmd = CustomCMD::update_command_with_variables(&cmd, &vars, &args);
        assert_eq!(new_cmd.args[1], ".".to_string());
    }

    #[test]
    fn test_update_command_with_variables_when_cmd_is_variable_then_replace_with_args_value() {
        let cmd = CMD { 
            cmd: "[[CMD]]".to_string(), 
            args: vec!["-l".to_string()]
        };
        let vars = vec![
            CMDVar {
                target: "[[CMD]]".to_string(),
                description: "test description".to_string()
            }
        ];
        let args = vec!["rt".to_string(), "testcmd".to_string(), "ls".to_string()];
        let new_cmd = CustomCMD::update_command_with_variables(&cmd, &vars, &args);
        assert_eq!(new_cmd.cmd, "ls".to_string());
    }
}
      */