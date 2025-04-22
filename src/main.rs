use std::env;

mod commands;
mod config;
mod system_commands;

use config::Config;
use system_commands::SystemCommandList;


// TODO:
// - Initialize app create default config
// - Replacable args
// - Add tests

fn main() {
    if cfg!(target_os = "windows") { panic!("No Windows support!") }
    let args: Vec<String> = env::args().collect();
    //dbg!(&args);

    let system_commands = SystemCommandList::new();

    let default_config_string = Config::default_json();
    let config: Config = serde_json::from_str(&default_config_string).unwrap();
    //dbg!("deserialized = {:?}", &config);
    
    process(&args, &config, &system_commands);

}

pub fn process(
    args: &Vec<String>, 
    config: &Config, 
    system_commands: &SystemCommandList
){
    if args.len() < 2 {
        print_help()
    } else {
        if !system_commands.run(&args[1]) {
            config.run(&args[1]);
        }
    }
}

fn print_help() {
    println!("Help:")
}