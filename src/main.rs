use std::{env, process::{self, Command}};

mod commands;
use commands::{CMD, CustomCMD};

fn main() {
    if cfg!(target_os = "windows") { panic!("No Windows support!") }
    let args: Vec<String> = env::args().collect();
    dbg!(args);

    let ccmd = CustomCMD {
        name: "test".to_string(),
        commands: vec![CMD::make_ls(), CMD::make_pwd()]
    };
    run_custom_command(ccmd);
}

fn run_custom_command(ccmd: CustomCMD) {
    for cmd in ccmd.commands {
        run_command(cmd);
    }
}

fn run_command(cmd: CMD) {
    let out = Command::new(cmd.command)
        .args(cmd.args)
        .output();
    let ok = out.unwrap_or_else(|error| {
        eprintln!("{error}");
        process::exit(1);
    });
    dbg!(ok);
}
