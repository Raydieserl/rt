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
    pub name: String,
    cmds: Vec<CMD>
}

impl CustomCMD {
    pub fn run(&self) {
        for cmd in &self.cmds {
            cmd.run();
        }
    }
}