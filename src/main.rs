use std::env;

mod file_cmds;
mod cmd_vars;
mod cmds_custom;
mod cmds_system;
mod exit;
mod help;

use cmd_vars::CMDVariablesTrait;
use cmds_custom::{CustomCMD, CustomCMDs, CustomCMDsTrait};
use cmds_system::{SystemCMD, SystemCMDs, SystemCMDsTrait};
use help::HelpProviding;

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

fn run(args: &Vec<String>, system_commands: &SystemCMDs, custom_commands: &CustomCMDs) {
    if let Some(cmd) = system_commands.run(args) {
        cmd.variables().exit_if_vars_do_not_match(&args);
        match cmd {
            SystemCMD::Help => help::print_help(vec![system_commands.help_item(), custom_commands.help_item()]),
            SystemCMD::Version => println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION")),
            SystemCMD::Shell => println!("{}", env!("SHELL")),
            SystemCMD::Export => file_cmds::export(&args),
            SystemCMD::Import => file_cmds::import(&args)
        }
    } else {
        custom_commands.run(args);
    }
}
