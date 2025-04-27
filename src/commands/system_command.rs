use super::command_variables::CommandVariable;

// System Commands
#[derive(Debug, PartialEq)]
pub enum SystemCommand {
    Help,
    Version,
    Shell,
    Export,
    Import,
    Remove,
    Add
}

impl SystemCommand {

    pub fn triggers(&self) -> Vec<String> {
        match self {
            Self::Help => vec!["-h".to_string(), "--help".to_string(), "help".to_string()],
            Self::Version => vec!["-v".to_string(), "--version".to_string(), "version".to_string()],
            Self::Shell => vec!["shell".to_string()],
            Self::Export => vec!["export".to_string()],
            Self::Import => vec!["import".to_string()],
            Self::Remove => vec!["remove".to_string()],
            Self::Add => vec!["add".to_string()]
        }
    }

    pub fn description(&self) -> Option<String> {
        Some(
            match self {
                Self::Help => "Shows help".to_string(),
                Self::Version => "Shows version".to_string(),
                Self::Shell => "Which shell will be used".to_string(),
                Self::Export => "Create a backup file for the custom commands".to_string(),
                Self::Import => "Import custom commands backup file".to_string(),
                Self::Remove => "Remove a command".to_string(),
                Self::Add => "Add command".to_string()
            }
        )
    }

    pub fn variables(&self) -> Option<Vec<CommandVariable>> {
        Some(
            match self {
                Self::Help => vec![],
                Self::Version => vec![],
                Self::Shell => vec![],
                Self::Export => vec![CommandVariable{target: "<FILE_PATH>".to_string(), description: "Path to backup file e.g. commands.backup.json".to_string()}],
                Self::Import => vec![CommandVariable{target: "<FILE_PATH>".to_string(), description: "Path to backup file e.g. commands.backup.json".to_string()}],
                Self::Remove => vec![CommandVariable{target: "<COMMAND>".to_string(), description: "Command to remove".to_string()}],
                Self::Add => vec![
                    CommandVariable{target: "<TRIGGER>".to_string(), description: "Trigger/name to execute command".to_string()},
                    CommandVariable{target: "<COMMAND>".to_string(), description: "Command to remove".to_string()}
                ]
            }
        )
    }
}
