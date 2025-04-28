use crate::commands::command_trait::CommandTrait;
use crate::commands::command_variables::CommandVariable;
use crate::commands::custom_command::CustomCommand;
use crate::commands::custom_commands::CustomCommands;
use crate::commands::system_command::SystemCommand;
use crate::commands::system_commands::SystemCommands;
use crate::help::{HelpItem, HelpItemCommand, HelpItemCommandProviding, HelpItemCommandVar, HelpItemGroup, HelpProviding};


impl HelpItemCommandProviding for CustomCommand {
    fn help_item_command(&self) -> HelpItemCommand {
        HelpItemCommand {
            triggers: self.triggers().clone(),
            description: self.description().unwrap_or(&"".to_string()).clone(),
            variables: as_help_item_command_variables(&self.variables())
        }
    }
}

impl HelpProviding for CustomCommands {
    fn help_item(&self) -> HelpItem {
        let mut group_names: Vec<String> = self.iter()
            .map(|e| e.groups().unwrap_or(&vec![]).clone())
            .flat_map(|e| e)
            .collect();
        group_names.sort();
        group_names.dedup();

        let no_group = HelpItemGroup {
            title: "".to_string(),
            commands: group_filter(&self, |e| e.groups().as_ref() == None)
        };

        let mut named_groups: Vec<HelpItemGroup> = group_names.iter().map(
            |n| HelpItemGroup {
                title: n.to_string(),
                commands: group_filter(&self, |e| e.groups().unwrap_or(&vec![]).contains(n))
            }
        ).collect();

        let mut groups: Vec<HelpItemGroup> = vec![no_group];
        groups.append(&mut named_groups);

        HelpItem {
            title: "CUSTOM COMMANDS:".to_string(),
            groups
        }
    }

}

fn group_filter<P>(cc: &CustomCommands, predicate: P) -> Vec<HelpItemCommand> where P: FnMut(&&CustomCommand) -> bool {
    cc.iter()
        .filter(predicate)
        .map(|e|e.help_item_command())
        .collect()
}

impl HelpItemCommandProviding for SystemCommand {
    fn help_item_command(&self) -> HelpItemCommand {
        HelpItemCommand {
            triggers: self.triggers().clone(),
            description: self.description().unwrap_or(&"".to_string()).clone(),
            variables: as_help_item_command_variables(&self.variables())
        }
    }
}

impl HelpProviding for SystemCommands {
    fn help_item(&self) -> HelpItem {
        /*HelpItem {
            title: "System Commands: ".to_string(),
            commands: self.iter().map(|e|e.help_item_command()).collect()
        }*/
        let groups = vec![];
        HelpItem {
            title: "SYSTEM COMMANDS:".to_string(),
            groups: groups
        }
    }
}

fn as_help_item_command_variables(variables: &Option<&Vec<CommandVariable>>) -> Vec<HelpItemCommandVar> {
    if let Some(var) = variables {
        return var.iter().map(
            |v| HelpItemCommandVar { 
                name: v.target.clone(), 
                description: v.description.clone() 
            }
        ).collect()
    }
    vec![]
}