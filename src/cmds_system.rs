use crate::help::{HelpProviding, HelpItem};

// System Commands
#[derive(Debug, PartialEq)]
pub enum SystemCMD {
    Help,
    List
}

impl SystemCMD {

    fn flags(&self) -> Vec<String> {
        match self {
            Self::Help => vec!["-h".to_string(), "--help".to_string(), "help".to_string()],
            Self::List => vec!["-l".to_string(), "--list".to_string(), "list".to_string()]
        }
    }

    fn description(&self) -> String {
        match self {
            Self::Help => "Shows help".to_string(),
            Self::List => "Lists custom commands".to_string()
        }
    }
}

// System Commands List
#[derive(Debug)]
pub struct SystemCMDList {
    cmds: Vec<SystemCMD>
}

impl SystemCMDList {

    pub fn new() -> SystemCMDList {
        SystemCMDList {
            cmds: vec![
                SystemCMD::Help,
                SystemCMD::List
            ]
        }
    }

    pub fn run(&self, args: &Vec<String>) -> Option<&SystemCMD> {
        for cmd in &self.cmds {
            if args.len() < 2 || cmd.flags().contains(&args[1]) {
                return Some(cmd)
            }
        }
        None
    }
}

impl HelpProviding for SystemCMDList {
    fn list_help_items(&self) -> Vec<HelpItem> {
        let mut help_items = vec![];
        for cmd in &self.cmds {
            help_items.push(
                HelpItem {
                    name: cmd.flags().join(", "),
                    description: cmd.description()
                }
            );
        }
        help_items
    }
}