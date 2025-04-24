use std::process::{self, Command};

use serde::{Serialize, Deserialize};

use crate::help::{HelpItemCMD, HelpItemVar, HelpProviding};


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
        let mut cmd_string = self.commands.join(" && ");
        if args.len() < self.variables.len() + 2 {
            let args: Vec<String> = self.variables.iter().map(|e| e.target.clone()).collect();
            eprintln!("Missing arguments for command: {}", args.join(", "));
            process::exit(1);
        }
        for (i, var) in self.variables.iter().enumerate() {
            cmd_string = cmd_string.replace(&var.target, &args[i+2]);
        }
        let out = Command::new("/bin/bash")
            .arg("-c")
            .arg(cmd_string)
            .output();
            let ok = out.unwrap_or_else(|error| {
                eprintln!("{}", error);
                process::exit(1);
            });
            println!("{}", String::from_utf8(ok.stdout).unwrap());
    }
}

// CMD Var
#[derive(Serialize, Deserialize, Debug)]
struct CMDVariable {
    description: String,
    target: String
}


// CustomCMDs
pub trait CustomCMDs {
    fn run(&self, args: &Vec<String>);
}

impl CustomCMDs for Vec<CustomCMD> {
    fn run(&self, args: &Vec<String>) {
        for ccmd in self {
            if ccmd.names.contains(&args[1]) {
                ccmd.run(args);
                return;
            }  
        } 
        eprintln!("Custom command not found: {}", args[1]);
        process::exit(1);
    }
}

impl HelpProviding for Vec<CustomCMD> {

    fn help_provider_title(&self) -> String {
        "Custom Commands: ".to_string()
    }

    fn list_help_items(&self) -> Vec<HelpItemCMD> {
        let mut help_items = vec![];
        for ccmd in self {
            let variables = ccmd.variables.iter().map(
                |v| HelpItemVar { name: v.target.clone(), description: v.description.clone() }
            ).collect();
            help_items.push(
                HelpItemCMD {
                    names: ccmd.names.clone(),
                    description: ccmd.description.clone(),
                    variables
                }
            );
        }
        help_items
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