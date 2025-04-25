use std::{fs::{self, File}, io::Write, path::{Path, PathBuf}, str::FromStr};

use crate::cmds_system::SystemCMD;

pub fn first_run() -> String {
    let (path_rt_dir, path_cmds_file) = paths();
    if !path_rt_dir.exists() {
        fs::create_dir(&path_rt_dir).unwrap();
    }
    if !path_cmds_file.exists() {
        let mut file = File::create(&path_cmds_file).unwrap();
        let default_config_string = default_config_string();
        file.write_all(default_config_string.as_bytes()).unwrap();
    }
    fs::read_to_string(&path_cmds_file).unwrap()
}

pub fn export(args: &Vec<String>) {
    let (_, path_cmds_file) = paths();
    let path_backup_file = PathBuf::from_str(&args[2]).unwrap();
    update_file(&path_cmds_file, &path_backup_file);
}

pub fn import(args: &Vec<String>) {
    let (_, path_cmds_file) = paths();
    let path_backup_file = PathBuf::from_str(&args[2]).unwrap();
    update_file(&path_backup_file, &path_cmds_file);
}

pub fn print_parse_error(args: &Vec<String>, error: &String) {
    if let Some(str) = args.get(1) { 
        if SystemCMD::Import.names().contains(str) { return }
    }
    let (_, path_cmds_file) = paths();
    let sep = "!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!";
    println!("\n{sep}");
    println!(
        "Parsing of {} failed\nError: {}\nFix: Try importing a backup or fix the error manually.", 
        path_cmds_file.into_os_string().into_string().unwrap(), 
        error
    );
    println!("{sep}\n");
}

fn update_file(in_path: &PathBuf, out_path: &PathBuf) {
    let config_string = fs::read_to_string(in_path).unwrap();
    let mut file = File::create(out_path).unwrap();
    file.write_all(config_string.as_bytes()).unwrap();
}

fn paths() -> (PathBuf, PathBuf) {
    let home = std::env::var("HOME").unwrap();
    let path_rt_dir = Path::new(&home).join(".rt");
    let path_cmds_file = path_rt_dir.join("commands.json");
    (path_rt_dir, path_cmds_file)
}

pub fn default_config_string() -> String {
r#"[
    {
        "names": ["testcmd"],
        "description": "This is just a test command",
        "commands": [
            "ls <PATH>"
        ],
        "variables": [
            {
                "target": "<PATH>",
                "description": "Path for ls"
            }
        ]
    }
]"#.to_string()
}