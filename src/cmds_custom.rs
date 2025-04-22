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
    pub fn run(&self) {
        for cmd in &self.cmds {
            cmd.run();
        }
    }
    /*
    fn update_command_with_variables(&self, cmd: &CMD) {
        let mut new_cmd: String = cmd.cmd.clone();
        for var in &self.vars {
            if cmd.cmd == var.target {

            }
        }
    }
    */
}

// CMD Var
#[derive(Serialize, Deserialize, Debug)]
struct CMDVar {
    description: String,
    target: String,
    default: String
}