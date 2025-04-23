use std::process::{self, Command};
use serde::{Serialize, Deserialize};

// Base Command
#[derive(Serialize, Deserialize, Debug)]
struct CMD {
    cmd: String,
    args: Vec<String>
}

impl CMD {
    fn run(&self) {
        let out = Command::new(&self.cmd)
        .args(&self.args)
        .output();
        let ok = out.unwrap_or_else(|error| {
            eprintln!("{error}");
            process::exit(1);
        });
        dbg!(ok);
    }
}

// Custom Commands
#[derive(Serialize, Deserialize, Debug)]
pub struct CustomCMD {
    pub names: Vec<String>,
    pub description: String,
    cmds: Vec<CMD>,
    vars: Vec<CMDVar>
}

impl CustomCMD {
    pub fn run(&self, args: &Vec<String>) {
        for cmd in &self.cmds {
            let new_cmd = CustomCMD::update_command_with_variables(&cmd, &self.vars, args);
            new_cmd.run();
        }
    }
}

impl CustomCMD {
    fn update_command_with_variables(cmd: &CMD, vars: &Vec<CMDVar>, args: &Vec<String>) -> CMD {
        if args.len() < vars.len() + 2 {
            let args: Vec<String> = vars.iter().map(|e| e.target.clone()).collect();
            eprintln!("Missing arguments for command: {}", args.join(", "));
            process::exit(1);
        }
        let mut new_cmd: String = cmd.cmd.clone();
        let mut new_args: Vec<String> = cmd.args.clone();
        for (i, var) in vars.iter().enumerate() {
            if cmd.cmd == var.target {
                new_cmd = args[i+2].clone();
            } else {
                for (ii, arg) in cmd.args.iter().enumerate() {
                    if arg == &var.target {
                        new_args[ii] = args[i+2].clone();
                    }
                }
            }
        }
        CMD { 
            cmd: new_cmd, 
            args: new_args
        }
    }
}

// CMD Var
#[derive(Serialize, Deserialize, Debug)]
struct CMDVar {
    description: String,
    target: String
}

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