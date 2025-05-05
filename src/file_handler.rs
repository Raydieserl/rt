use std::{fs::{self, File}, io::Write, path::{Path, PathBuf}, str::FromStr};

use crate::{commands::{command_trait::CommandTrait, command_variables::CommandVariable, custom_command::CustomCommand, custom_commands::CustomCommands}, file_json};

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
        ],
        "groups": ["TestGroup"]
    }
]"#.to_string()
        }
    }
}

impl FileHandler {
    pub fn make_custom_commands(&self) -> CustomCommands {
        if !&self.path_rt_dir.exists() {
            fs::create_dir(&self.path_rt_dir).unwrap();
        }
        if !&self.path_commands_file.exists() {
            let mut file = File::create(&self.path_commands_file).unwrap();
            file.write_all(&self.default_config_string.as_bytes()).unwrap();
        }
        let custom_commands = fs::read_to_string(&self.path_commands_file).unwrap();
        file_json::deserialize(
            &custom_commands, 
            &self.default_config_string
        )
    }
    
    pub fn script(&self, path: &String, custom_command: &CustomCommand) {
        let mut variables: Vec<&CommandVariable> = vec![];
        let mut text = format!("#!{}\n\n", env!("SHELL"));
        text.push_str(&format!("# DESCRIPTION:\n"));
        text.push_str(&format!("#   {}\n", custom_command.description().unwrap_or(&"".to_string())));
        if let Some(var) = custom_command.variables() {
            text.push_str(&format!("# VARIABLES:\n"));
            for (i, v) in var.iter().enumerate() {
                text.push_str(&format!("#   ${} {}: {}\n", i+1, v.target, v.description));
                variables.push(v);
            }
        }
        text.push_str(&format!("\n"));
        for c in &custom_command.commands {
            let mut command = format!("{}\n", c);
            for (i, v) in variables.iter().enumerate() {
                command = command.replace(&v.target, &format!("${}", i+1));
            }
            text.push_str(&command);
        }
        let mut file = File::create(path).unwrap();
        file.write_all(text.as_bytes()).unwrap();
    }
    
    pub fn safe(&self, custom_commands: &CustomCommands) {
        let json = file_json::serialize(custom_commands);
        let mut file = File::create(&self.path_commands_file).unwrap();
        file.write_all(json.as_bytes()).unwrap();
    }

    pub fn export(&self, path: &String) {
        let path_backup_file = PathBuf::from_str(path).unwrap();
        self.update_file(&self.path_commands_file, &path_backup_file);
    }

    pub fn import(&self, path: &String) {
        let path_backup_file = PathBuf::from_str(path).unwrap();
        self.update_file(&path_backup_file, &self.path_commands_file);
    }

    fn update_file(&self, in_path: &PathBuf, out_path: &PathBuf) {
        let config_string = fs::read_to_string(in_path).unwrap();
        let mut file = File::create(out_path).unwrap();
        file.write_all(config_string.as_bytes()).unwrap();
    }

}