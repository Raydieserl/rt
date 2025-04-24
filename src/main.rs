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
// - define shell to use
// - fish help

fn main() {
    if cfg!(target_os = "windows") { panic!("No Windows support!") }
    let args: Vec<String> = env::args().collect();
    
    let config_as_string = first_run(); // Config::default_json();
    let system_commands = SystemCMDList::new();
    let config: Config = serde_json::from_str(&config_as_string).unwrap();
    //dbg!(&config);
    
    run(&args, config, system_commands);
}

fn first_run() -> String {
    let home = std::env::var("HOME").unwrap();
    let path_home = Path::new(&home).join(".rt");
    let path_file = path_home.join("config.json");
    
    if !fs::exists(&path_home).unwrap() {
        fs::create_dir(&path_home).unwrap();
        let mut file = File::create(&path_file).unwrap();
        let default_config_string = Config::default_json();
        file.write_all(default_config_string.as_bytes()).unwrap();
    }

    fs::read_to_string(&path_file).unwrap()
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
