use std::{fs::{self, File}, io::Write, path::{Path, PathBuf}, str::FromStr};

use crate::{commands::custom_commands::CustomCommands, file_json};

pub struct FileHandler {
    path_rt_dir: PathBuf,
    path_commands_file: PathBuf,
    default_config_string: String
}

impl FileHandler {
    pub fn new() -> FileHandler {
        let home = std::env::var("HOME").unwrap();
        let path_rt_dir = Path::new(&home).join(".rt");
        let path_commands_file = path_rt_dir.join("commands.json");
        FileHandler { 
            path_rt_dir, 
            path_commands_file, 
            default_config_string: r#"[
    {
        "triggers": ["testcommand"],
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
    }
}

impl FileHandler {
    pub fn make_custom_commands(&self, args: &Vec<String>) -> CustomCommands {
        if !&self.path_rt_dir.exists() {
            fs::create_dir(&self.path_rt_dir).unwrap();
        }
        if !&self.path_commands_file.exists() {
            let mut file = File::create(&self.path_commands_file).unwrap();
            file.write_all(&self.default_config_string.as_bytes()).unwrap();
        }
        let custom_commands = fs::read_to_string(&self.path_commands_file).unwrap();
        file_json::deserialize(
            &args, 
            &custom_commands, 
            &self.default_config_string
        )
    }

    pub fn safe(&self, custom_commands: &CustomCommands) {
        let json = file_json::serialize(&custom_commands);
        let mut file = File::create(&self.path_commands_file).unwrap();
        file.write_all(json.as_bytes()).unwrap();
    }

    pub fn export(&self, args: &Vec<String>) {
        let path_backup_file = PathBuf::from_str(&args[2]).unwrap();
        self.update_file(&self.path_commands_file, &path_backup_file);
    }

    pub fn import(&self, args: &Vec<String>) {
        let path_backup_file = PathBuf::from_str(&args[2]).unwrap();
        self.update_file(&path_backup_file, &self.path_commands_file);
    }

    fn update_file(&self, in_path: &PathBuf, out_path: &PathBuf) {
        let config_string = fs::read_to_string(in_path).unwrap();
        let mut file = File::create(out_path).unwrap();
        file.write_all(config_string.as_bytes()).unwrap();
    }

}