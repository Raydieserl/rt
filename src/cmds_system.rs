use crate::cmd_vars::{CMDVariable, CMDVariablesTrait};
use crate::help::{HelpItem, HelpItemCMD, HelpItemCMDProviding, HelpProviding};

// System Commands
#[derive(Debug, PartialEq)]
pub enum SystemCMD {
    Help,
    Version,
    Shell,
    Export,
    Import,
    Remove
}

impl SystemCMD {

    pub fn names(&self) -> Vec<String> {
        match self {
            Self::Help => vec!["-h".to_string(), "--help".to_string(), "help".to_string()],
            Self::Version => vec!["-v".to_string(), "--version".to_string(), "version".to_string()],
            Self::Shell => vec!["shell".to_string()],
            Self::Export => vec!["export".to_string()],
            Self::Import => vec!["import".to_string()],
            Self::Remove => vec!["remove".to_string()]
        }
    }

    fn description(&self) -> String {
        match self {
            Self::Help => "Shows help".to_string(),
            Self::Version => "Shows version".to_string(),
            Self::Shell => "Which shell will be used".to_string(),
            Self::Export => "Create a backup file for the custom commands".to_string(),
            Self::Import => "Import custom commands backup file".to_string(),
            Self::Remove => "Remove a command".to_string()
        }
    }

    pub fn variables(&self) -> Vec<CMDVariable> {
        match self {
            Self::Help => vec![],
            Self::Version => vec![],
            Self::Shell => vec![],
            Self::Export => vec![CMDVariable{target: "<FILE_PATH>".to_string(), description: "Path to backup file e.g. commands.backup.json".to_string()}],
            Self::Import => vec![CMDVariable{target: "<FILE_PATH>".to_string(), description: "Path to backup file e.g. commands.backup.json".to_string()}],
            Self::Remove => vec![CMDVariable{target: "<COMMAND_NAME>".to_string(), description: "Name of command to remove".to_string()}]
        }
    }
}

impl HelpItemCMDProviding for SystemCMD {
    fn help_item_cmd(&self) -> HelpItemCMD {
        HelpItemCMD {
            names: self.names(),
            description: self.description(),
            variables: self.variables().as_help_item_cmd_vars()
        }
    }
}


// SystemCMDs
pub const SYSTEM_CMDS: [SystemCMD; 6] = [
    SystemCMD::Help,
    SystemCMD::Version,
    SystemCMD::Shell,
    SystemCMD::Export,
    SystemCMD::Import,
    SystemCMD::Remove
];

pub type SystemCMDs = [SystemCMD];
pub trait SystemCMDsTrait {
    fn run(&self, args: &Vec<String>) -> Option<&SystemCMD>;
}
impl SystemCMDsTrait for SystemCMDs {
    fn run(&self, args: &Vec<String>) -> Option<&SystemCMD> {
        self.iter().find(|cmd| args.len() < 2 || cmd.names().contains(&args[1]))
    }
}

impl HelpProviding for SystemCMDs {
    fn help_item(&self) -> crate::help::HelpItem {
        HelpItem {
            title: "System Commands: ".to_string(),
            commands: self.iter().map(|e|e.help_item_cmd()).collect()
        }
    }
}