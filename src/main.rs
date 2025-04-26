use std::env;

mod cmd_vars;
mod cmds_custom;
mod cmds_system;
mod exit;
mod file_handler;
mod help;
mod file_json;

use cmd_vars::CMDVariablesTrait;
use cmds_custom::{CustomCMD, CustomCMDs, CustomCMDsTrait};
use cmds_system::{SystemCMD, SystemCMDs, SystemCMDsTrait};
use file_handler::FileHandler;
use help::HelpProviding;

fn main() {
    if cfg!(target_os = "windows") { panic!("No Windows support!") }
    let args: Vec<String> = env::args().collect();
    
    let file_handler = FileHandler::new();
    let system_commands = cmds_system::SYSTEM_CMDS;
    let mut custom_commands: Vec<CustomCMD> = file_handler.make_custom_commands(&args);
    
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
    system_commands: &SystemCMDs, 
    custom_commands: &mut CustomCMDs
) {
    if let Some(cmd) = system_commands.run(args) {
        cmd.variables().exit_if_vars_do_not_match(&args);
        match cmd {
            SystemCMD::Help => help::print_help(vec![system_commands.help_item(), custom_commands.help_item()]),
            SystemCMD::Version => println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION")),
            SystemCMD::Shell => println!("{}", env!("SHELL")),
            SystemCMD::Export => file_handler.export(&args),
            SystemCMD::Import => file_handler.import(&args),
            SystemCMD::Remove => {
                custom_commands.remove_by_name(args);
                file_handler.safe(&custom_commands);
            }
        }
    } else {
        custom_commands.run(args);
    }
}
