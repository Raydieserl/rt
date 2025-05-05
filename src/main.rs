use std::env;

mod commands;
mod file_handler;
mod help;
mod help_providing_impl;
mod file_json;

use commands::command_trait::CommandTrait;
use commands::command_variables::CommandVariablesTrait;
use commands::custom_commands::{CustomCommands, CustomCommandsTrait};
use commands::system_command::SystemCommandType;
use commands::system_commands::{SystemCommands, SystemCommandsTrait};
use file_handler::FileHandler;
use help::HelpProviding;

fn main() {
    if cfg!(target_os = "windows") { panic!("No Windows support!") }

    let args: Vec<String> = env::args().collect();
    let trigger_default = "".to_string();
    let trigger = args.get(1).unwrap_or(&trigger_default);
    let parameters: Vec<&String> = args.iter().enumerate().filter(|(i,_)|*i>1).map(|(_,e)|e).collect();
    
    let file_handler = FileHandler::new();
    let system_commands = SystemCommands::make();
    let mut custom_commands: CustomCommands = file_handler.make_custom_commands();
    
    run(
        trigger, 
        &parameters,
        &file_handler, 
        &system_commands, 
        &mut custom_commands
    );
}

fn run(
    trigger: &String, 
    parameters: &Vec<&String>,
    file_handler: &FileHandler, 
    system_commands: &SystemCommands, 
    custom_commands: &mut CustomCommands
) {
    let help = || {
        help::print_help(vec![system_commands.help_item(), custom_commands.help_item()])
    };
    if trigger.is_empty() {
        help();
    } else if let Some(command) = system_commands.run(trigger) {
        command.variables().unwrap_or(&vec![]).exit_if_vars_do_not_match(&parameters);
        match command.command_type {
            SystemCommandType::Help => help(),
            SystemCommandType::Version => println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION")),
            SystemCommandType::Shell => println!("{}", env!("SHELL")),
            SystemCommandType::Export => file_handler.export(&parameters[0]),
            SystemCommandType::Import => file_handler.import(&parameters[0]),
            SystemCommandType::Remove => {
                custom_commands.remove_by_trigger(&parameters[0]);
                file_handler.safe(custom_commands);
            },
            SystemCommandType::Add => {
                custom_commands.add_command(&parameters[0], &parameters[1]);
                file_handler.safe(custom_commands);
            },
            SystemCommandType::Script => {
                if let Some((_, custom_command)) = custom_commands.get_idx_and_item(&parameters[0]) {
                    file_handler.script(&parameters[1], custom_command)
                }
            }
        }
    } else {
        custom_commands.run(trigger, &parameters);
    }
}