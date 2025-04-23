use std::{env, fs::{self, File}, io::Write, path::Path};

mod cmds_custom;
mod cmds_system;
mod config;
mod help;

use config::Config;
use cmds_system::{SystemCMD, SystemCMDList};


// TODO:
// - Initialize app create default config
// - Add tests
// - error handling
// - cloning

fn main() {
    if cfg!(target_os = "windows") { panic!("No Windows support!") }
    let args: Vec<String> = env::args().collect();
    let default_config_string = Config::default_json();

   first_run(&default_config_string);

    let system_commands = SystemCMDList::new();
    let config: Config = serde_json::from_str(&default_config_string).unwrap();
    
    run(&args, config, system_commands);
}

fn first_run(default_config_string: &String) {
    let home = std::env::var("HOME").unwrap();
    let path = Path::new(&home).join(".rt");
    
    if !fs::exists(&path).unwrap() {
        fs::create_dir(&path).unwrap();
        let mut file = File::create(path.join("config.json")).unwrap();
        file.write_all(default_config_string.as_bytes()).unwrap();
    }
}

fn run(args: &Vec<String>, config: Config, system_commands: SystemCMDList) {
    if let Some(cmd) = system_commands.run(args) {
        match cmd {
            SystemCMD::Help => help::print_help(vec![Box::new(system_commands), Box::new(config)]),
            SystemCMD::List => println!("List")
        }
    } else {
        config.run(args);
    }
}