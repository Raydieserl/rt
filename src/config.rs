use std::process;

use serde::{Serialize, Deserialize};

use crate::{cmds_custom::CustomCMD, help::{HelpProviding, HelpItem}};

// Config
#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    custom_cmds: Vec<CustomCMD>
}

impl Config {
    pub fn run(&self, args: &Vec<String>) {
        for ccmd in &self.custom_cmds {
            if ccmd.names.contains(&args[1]) {
                ccmd.run(args);
            } else {
                eprintln!("Custom command not found: {}", args[1]);
                process::exit(1);
            }
        } 
    }
}

impl Config {
    pub fn default_json() -> String {
r#"{
    "custom_cmds": [
        {
            "names": ["testcmd"],
            "description": "This is just a test command",
            "cmds": [
                {
                    "cmd": "ls",
                    "args": ["-l", "-a", "<PATH>"]
                },
                {
                    "cmd": "pwd",
                    "args": []
                }
            ],
            "vars": [
                {
                    "target": "<PATH>",
                    "description": "Path for ls"
                }
            ]
        }
    ]
}"#.to_string()
    }
}

impl HelpProviding for Config {

    fn help_provider_title(&self) -> String {
        "Custom Commands: ".to_string()
    }

    fn list_help_items(&self) -> Vec<HelpItem> {
        let mut help_items = vec![];
        for ccmd in &self.custom_cmds {
            help_items.push(
                HelpItem {
                    names: ccmd.names.clone(),
                    description: ccmd.description.clone()
                }
            );
        }
        help_items
    }

}