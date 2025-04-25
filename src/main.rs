use std::env;

mod file_cmds;
mod cmds_custom;
mod cmds_system;
mod help;

use cmds_custom::{CustomCMD, CustomCMDs};
use cmds_system::{SystemCMD, SystemCMDs, SystemCMDsTrait};
use help::HelpProviding;


// TODO:
// Improvements:
//   - Add tests
//   - error handling
//   - cloning
// Features:
// README.md
//   - remove cmd
//   - add cmd
//   - export as .sh script
// Future:
//   - change shell
//   - windows support
//   - groups for commands

fn main() {
    if cfg!(target_os = "windows") { panic!("No Windows support!") }
    let args: Vec<String> = env::args().collect();
    
    let system_commands = cmds_system::SYSTEM_CMDS;
    let custom_commands: Vec<CustomCMD> = serde_json::from_str(
        &file_cmds::first_run()
    ).unwrap_or_else(|error| {
        file_cmds::print_parse_error(&args, &error.to_string());
        serde_json::from_str(&file_cmds::default_config_string()).unwrap()
    });
    
    run(&args, &system_commands, &custom_commands);
}

fn run(args: &Vec<String>, system_commands: &SystemCMDs, custom_commands: &Vec<CustomCMD>) {
    if let Some(cmd) = system_commands.run(args) {
        match cmd {
            SystemCMD::Help => help::print_help(vec![system_commands.help_item(), custom_commands.help_item()]),
            SystemCMD::Version => println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION")),
            SystemCMD::Shell => println!("{}", env!("SHELL")),
            SystemCMD::Export => file_cmds::export(),
            SystemCMD::Import => file_cmds::import()
        }
    } else {
        custom_commands.run(args);
    }
}
