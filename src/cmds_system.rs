use crate::help::{HelpItem, HelpItemCMD, HelpItemCMDProviding, HelpProviding};

// System Commands
#[derive(Debug, PartialEq)]
pub enum SystemCMD {
    Help,
    Version,
    Shell,
    Export,
    Import
}

impl SystemCMD {

    pub fn names(&self) -> Vec<String> {
        match self {
            Self::Help => vec!["-h".to_string(), "--help".to_string(), "help".to_string()],
            Self::Version => vec!["-v".to_string(), "--version".to_string(), "version".to_string()],
            Self::Shell => vec!["shell".to_string()],
            Self::Export => vec!["export".to_string()],
            Self::Import => vec!["import".to_string()]
        }
    }

    fn description(&self) -> String {
        match self {
            Self::Help => "Shows help".to_string(),
            Self::Version => "Shows version".to_string(),
            Self::Shell => "Which shell will be used".to_string(),
            Self::Export => "Create a commands.json.backup file in current directory".to_string(),
            Self::Import => "Import commands.json.backup from current directory".to_string()
        }
    }
}

impl HelpItemCMDProviding for SystemCMD {
    fn help_item_cmd(&self) -> HelpItemCMD {
        HelpItemCMD {
            names: self.names(),
            description: self.description(),
            variables: vec![]
        }
    }
}


// SystemCMDs
pub const SYSTEM_CMDS: [SystemCMD; 5] = [
    SystemCMD::Help,
    SystemCMD::Version,
    SystemCMD::Shell,
    SystemCMD::Export,
    SystemCMD::Import
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