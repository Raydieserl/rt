use crate::commands::{command_variables::CommandVariable, system_command::SystemCommandType};

use super::{command_trait::CommandTrait, system_command::SystemCommand};

pub type SystemCommands = Vec<SystemCommand>;
pub trait SystemCommandsTrait {
    fn make() -> SystemCommands {
        vec![
            SystemCommand::new(
                SystemCommandType::Help,
                vec!["-h".to_string(), "--help".to_string(), "help".to_string()],
                Some("Shows help".to_string()),
                None,
                None
            ),
            SystemCommand::new(
                SystemCommandType::Version,
                vec!["-v".to_string(), "--version".to_string(), "version".to_string()],
                Some( "Shows version".to_string()),
                None,
                None
            ),
            SystemCommand::new(
                SystemCommandType::Shell,
                vec!["shell".to_string()],
                Some("Which shell will be used".to_string()),
                None,
                None
            ),
            SystemCommand::new(
                SystemCommandType::Export,
                vec!["export".to_string()],
                Some("Create a backup file for the custom commands".to_string()),
                Some(vec![CommandVariable{target: "<FILE_PATH>".to_string(), description: "Path to backup file e.g. commands.backup.json".to_string()}]),
                Some(vec!["Backup".to_string()])
            ),
            SystemCommand::new(
                SystemCommandType::Import,
                vec!["import".to_string()],
                Some("Import custom commands backup file".to_string()),
                Some(vec![CommandVariable{target: "<FILE_PATH>".to_string(), description: "Path to backup file e.g. commands.backup.json".to_string()}]),
                Some(vec!["Backup".to_string()])
            ),
            SystemCommand::new(
                SystemCommandType::Remove,
                vec!["remove".to_string()],
                Some("Remove a command".to_string()),
                Some(vec![CommandVariable{target: "<COMMAND>".to_string(), description: "Command to remove".to_string()}]),
                Some(vec!["Edit".to_string()])
            ),
            SystemCommand::new(
                SystemCommandType::Add,
                vec!["add".to_string()],
                Some("Add a command".to_string()),
                Some(vec![
                    CommandVariable{target: "<TRIGGER>".to_string(), description: "Trigger/name to execute command".to_string()},
                    CommandVariable{target: "<COMMAND>".to_string(), description: "Command to remove".to_string()}
                ]),
                Some(vec!["Edit".to_string()])
            )
        ]
    }
    fn run(&self, args: &Vec<String>) -> Option<&SystemCommand>;
}

impl SystemCommandsTrait for SystemCommands {
    fn run(&self, args: &Vec<String>) -> Option<&SystemCommand> {
        self.iter().find(|command| args.len() < 2 || command.triggers().contains(&args[1]))
    }
}