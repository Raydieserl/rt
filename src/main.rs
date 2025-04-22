use std::env;

mod cmds_custom;
mod cmds_system;
mod config;
mod help;

use config::Config;
use cmds_system::{SystemCMD, SystemCMDList};


// TODO:
// - Initialize app create default config
// - Replacable args
// - Add tests

fn main() {
    if cfg!(target_os = "windows") { panic!("No Windows support!") }
    let args: Vec<String> = env::args().collect();
    //dbg!(&args);

    let system_commands = SystemCMDList::new();

    let default_config_string = Config::default_json();
    let config: Config = serde_json::from_str(&default_config_string).unwrap();
    //dbg!("deserialized = {:?}", &config);
    
    if let Some(cmd) = system_commands.run(&args) {
        match cmd {
            SystemCMD::Help => help::print_help(vec![Box::new(system_commands), Box::new(config)]),
            SystemCMD::List => println!("List")
        }
    } else {
        config.run(&args[1]);
    }
}