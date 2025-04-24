use std::process::{self, Command};

use serde::{Serialize, Deserialize};


// Custom Commands
#[derive(Serialize, Deserialize, Debug)]
pub struct CustomCMD {
    pub names: Vec<String>,
    pub description: String,
    commands: Vec<String>,
    pub variables: Vec<CMDVariable>
}

impl CustomCMD {
    pub fn run(&self, args: &Vec<String>) {
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
                eprintln!("{error}");
                process::exit(1);
            });
            dbg!(ok);
    }
}

// CMD Var
#[derive(Serialize, Deserialize, Debug)]
pub struct CMDVariable {
    pub description: String,
    pub target: String
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