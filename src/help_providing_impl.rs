use crate::commands::command_variables::CommandVariable;
use crate::commands::custom_command::CustomCommand;
use crate::commands::custom_commands::CustomCommands;
use crate::commands::system_command::SystemCommand;
use crate::commands::system_commands::SystemCommands;
use crate::help::{HelpItem, HelpItemCommand, HelpItemCommandProviding, HelpItemCommandVar, HelpProviding};


impl HelpItemCommandProviding for CustomCommand {
    fn help_item_command(&self) -> HelpItemCommand {
        HelpItemCommand {
            triggers: self.triggers.clone(),
            description: self.description.clone(),
            variables: as_help_item_command_variables(&self.variables)
        }
    }
}

impl HelpProviding for CustomCommands {
    fn help_item(&self) -> HelpItem {
        HelpItem {
            title: "Custom Commands: ".to_string(),
            commands: self.iter().map(|e|e.help_item_command()).collect()
        }
    }
}

impl HelpItemCommandProviding for SystemCommand {
    fn help_item_command(&self) -> HelpItemCommand {
        HelpItemCommand {
            triggers: self.triggers(),
            description: self.description(),
            variables: as_help_item_command_variables(&self.variables())
        }
    }
}

impl HelpProviding for SystemCommands {
    fn help_item(&self) -> HelpItem {
        HelpItem {
            title: "System Commands: ".to_string(),
            commands: self.iter().map(|e|e.help_item_command()).collect()
        }
    }
}

fn as_help_item_command_variables(variables: &Vec<CommandVariable>) -> Vec<HelpItemCommandVar> {
    variables.iter().map(
        |v| HelpItemCommandVar { 
            name: v.target.clone(), 
            description: v.description.clone() 
        }
    ).collect()
}