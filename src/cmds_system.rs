use crate::help::{HelpItemCMD, HelpProviding};

// System Commands
#[derive(Debug, PartialEq)]
pub enum SystemCMD {
    Help,
    Version,
    Export,
    Import
}

impl SystemCMD {

    fn names(&self) -> Vec<String> {
        match self {
            Self::Help => vec!["-h".to_string(), "--help".to_string(), "help".to_string()],
            Self::Version => vec!["-v".to_string(), "--version".to_string(), "version".to_string()],
            Self::Export => vec!["export".to_string()],
            Self::Import => vec!["import".to_string()]
        }
    }

    fn description(&self) -> String {
        match self {
            Self::Help => "Shows help".to_string(),
            Self::Version => "Shows version".to_string(),
            Self::Export => "Create a config.json.backup file in current directory".to_string(),
            Self::Import => "Import config.json.backup from current directory".to_string()
        }
    }
}

// System Commands List
#[derive(Debug)]
pub struct SystemCMDs {
    cmds: Vec<SystemCMD>
}

impl SystemCMDs {

    pub fn new() -> SystemCMDs {
        SystemCMDs {
            cmds: vec![
                SystemCMD::Help,
                SystemCMD::Version,
                SystemCMD::Export,
                SystemCMD::Import
            ]
        }
    }

    pub fn run(&self, args: &Vec<String>) -> Option<&SystemCMD> {
        for cmd in &self.cmds {
            if args.len() < 2 || cmd.names().contains(&args[1]) {
                return Some(cmd)
            }
        }
        None
    }
}

impl HelpProviding for SystemCMDs {
    
    fn help_provider_title(&self) -> String {
        "System Commands: ".to_string()
    }

    fn list_help_items(&self) -> Vec<HelpItemCMD> {
        let mut help_items = vec![];
        for cmd in &self.cmds {
            help_items.push(
                HelpItemCMD {
                    names: cmd.names(),
                    description: cmd.description(),
                    variables: vec![]
                }
            );
        }
        help_items
    }

}