use std::process;

use serde::{Serialize, Deserialize};

use crate::{cmds_custom::CustomCMD, help::{HelpItemCMD, HelpItemVar, HelpProviding}};

// Config
#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    version: String,
    custom_commands: Vec<CustomCMD>
}

impl Config {
    pub fn run(&self, args: &Vec<String>) {
        for ccmd in &self.custom_commands {
            if ccmd.names.contains(&args[1]) {
                ccmd.run(args);
                return;
            }  
        } 
        eprintln!("Custom command not found: {}", args[1]);
        process::exit(1);
    }
}

impl Config {
    pub fn default_json() -> String {
r#"{
    "version": "1",
    "custom_commands": [
        {
            "names": ["testcmd"],
            "description": "This is just a test command",
            "commands": [
                "ls <PATH>"
            ],
            "variables": [
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

    fn list_help_items(&self) -> Vec<HelpItemCMD> {
        let mut help_items = vec![];
        for ccmd in &self.custom_commands {
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
{
    "names": ["pyproj"],
    "description": "Create python project with venv and git",
    "commands": [
        "mkdir <PATH>",
        "cd <PATH>",
        "python3 -m venv .venv",
        "git init",
        "touch .gitignore",
        "touch main.py"
    ],
    "variables": [
        {
            "target": "<PATH>",
            "description": "Path for project e.g. ~/Desktop/test_pyproj"
        }
    ]
}
*/