use std::env;

mod config_types;
use config_types::Config;

fn default_config_string() -> String {
    r#"
        {
           "custom_cmds": [
                {
                    "name": "testcommand",
                    "cmds": [
                        {
                            "cmd": "ls",
                            "args": ["-l", "-a"]
                        },
                        {
                            "cmd": "pwd",
                            "args": []
                        }
                    ]
                }
           ]
        }
    "#.to_string()
}

fn main() {
    if cfg!(target_os = "windows") { panic!("No Windows support!") }
    let args: Vec<String> = env::args().collect();
    dbg!(&args);

    let default_config_string = default_config_string();
    let config: Config = serde_json::from_str(&default_config_string).unwrap();
    println!("deserialized = {:?}", config);

    config.run(&args[1]);
}