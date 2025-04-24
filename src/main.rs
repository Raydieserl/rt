use std::env;

mod config;
mod cmds_custom;
mod cmds_system;
mod help;

use cmds_custom::{CustomCMD, CustomCMDs};
use cmds_system::{SystemCMD, SystemCMDs};


// TODO:
// Improvements:
//   - Add tests
//   - error handling
//   - cloning
// Features:
//   - define shell to use
//   - install script

fn main() {
    if cfg!(target_os = "windows") { panic!("No Windows support!") }
    let args: Vec<String> = env::args().collect();
    
    let config_as_string = config::first_run();
    let system_commands = SystemCMDs::new();
    let custom_commands: Vec<CustomCMD> = serde_json::from_str(&config_as_string).unwrap();
    
    run(&args, system_commands, custom_commands);
}

fn run(args: &Vec<String>, system_commands: SystemCMDs, custom_commands: Vec<CustomCMD>) {
    if let Some(cmd) = system_commands.run(args) {
        match cmd {
            SystemCMD::Help => help::print_help(vec![Box::new(system_commands), Box::new(custom_commands)]),
            SystemCMD::Version => println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION")),
            SystemCMD::Export => config::export(),
            SystemCMD::Import => config::import()
        }
    } else {
        custom_commands.run(args);
    }
}
