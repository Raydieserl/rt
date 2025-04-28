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
    
    let file_handler = FileHandler::new();
    let system_commands = SystemCommands::make();
    let mut custom_commands: CustomCommands = file_handler.make_custom_commands();
    
    run(
        &args, 
        &file_handler, 
        &system_commands, 
        &mut custom_commands
    );
}

fn run(
    args: &Vec<String>, 
    file_handler: &FileHandler, 
    system_commands: &SystemCommands, 
    custom_commands: &mut CustomCommands
) {
    if let Some(command) = system_commands.run(args) {
        command.variables().unwrap_or(&vec![]).exit_if_vars_do_not_match(&args);
        match command.command_type {
            SystemCommandType::Help => help::print_help(vec![system_commands.help_item(), custom_commands.help_item()]),
            SystemCommandType::Version => println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION")),
            SystemCommandType::Shell => println!("{}", env!("SHELL")),
            SystemCommandType::Export => file_handler.export(&args),
            SystemCommandType::Import => file_handler.import(&args),
            SystemCommandType::Remove => {
                custom_commands.remove_by_trigger(args);
                file_handler.safe(&custom_commands);
            },
            SystemCommandType::Add => {
                custom_commands.add_command(args);
                file_handler.safe(&custom_commands);
            }
        }
    } else {
        custom_commands.run(args);
    }
}
