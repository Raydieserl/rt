use std::process;

use serde::{Serialize, Deserialize};

use crate::commands::CustomCMD;

// Config
#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    custom_cmds: Vec<CustomCMD>
}

impl Config {
    pub fn run(&self, cmd_name: &str) {
        for ccmd in &self.custom_cmds {
            if ccmd.name == cmd_name {
                ccmd.run();
            } else {
                eprintln!("Custom command not found: {}", cmd_name);
                process::exit(1);
            }
        } 
    }
}

impl Config {
    pub fn default_json() -> String {
        r#"
            {
            "custom_cmds": [
                    {
                        "name": "testcommand",
                        "cmds": [
                            {
                                "cmd": "ls",
                                "args": ["-l", "-a"]
                            },
                            {
                                "cmd": "pwd",
                                "args": []
                            }
                        ]
                    }
            ]
            }
        "#.to_string()
    }
}