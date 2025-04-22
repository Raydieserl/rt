// Base Command
#[derive(Debug)]
pub struct CMD {
    pub command: String,
    pub args: Vec<String>
}

impl CMD {
    
    pub fn make_ls() -> Self {
        CMD {
            command: "ls".to_string(),
            args: vec!["-l".to_string(), "-a".to_string()]
        }
    }

    pub fn make_pwd() -> Self {
        CMD {
            command: "pwd".to_string(),
            args: vec![]
        }
    }

}

// Custom Commands
#[derive(Debug)]
pub struct CustomCMD {
    pub name: String,
    pub commands: Vec<CMD>
}