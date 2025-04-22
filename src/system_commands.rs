// System Commands
#[derive(Debug)]
struct SystemCommand {
    flags: Vec<String>
}

impl SystemCommand {
    fn run(&self) {
        println!("RUN {:?}", self.flags)
    }
}

// System Commands List
#[derive(Debug)]
pub struct SystemCommandList {
    cmds: Vec<SystemCommand>
}

impl SystemCommandList {
    pub fn new() -> SystemCommandList {
        SystemCommandList {
            cmds: vec![
                SystemCommand {
                    flags: vec!["-h".to_string(), "--help".to_string(), "help".to_string()]
                }
            ]
        }
    }

    pub fn run(&self, flag: &String) -> bool {
        for cmd in &self.cmds {
            if cmd.flags.contains(flag) {
                cmd.run();
                return true
            }
        }
        false
    }

}