use std::process::{self, Command};

use serde::{Serialize, Deserialize};

// Base Command
#[derive(Serialize, Deserialize, Debug)]
pub struct CMD {
    pub cmd: String,
    pub args: Vec<String>
}

impl CMD {
    pub fn run(&self) {
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
    pub name: String,
    pub cmds: Vec<CMD>
}

impl CustomCMD {
    pub fn run(&self) {
        for cmd in &self.cmds {
            cmd.run();
        }
    }
}

// Config
#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub custom_cmds: Vec<CustomCMD>
}

impl Config {
    pub fn run(&self, cmd_name: &str) {
        for ccmd in &self.custom_cmds {
            if ccmd.name == cmd_name {
                ccmd.run();
            } else {
                eprintln!("Custom command not found {}", cmd_name);
                process::exit(1);
            }
        } 
    }
}