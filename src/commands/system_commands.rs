use super::system_command::SystemCommand;

// SystemCommands
pub const SYSTEM_COMMANDS: [SystemCommand; 6] = [
    SystemCommand::Help,
    SystemCommand::Version,
    SystemCommand::Shell,
    SystemCommand::Export,
    SystemCommand::Import,
    SystemCommand::Remove
];

pub type SystemCommands = [SystemCommand];
pub trait SystemCommandsTrait {
    fn run(&self, args: &Vec<String>) -> Option<&SystemCommand>;
}
impl SystemCommandsTrait for SystemCommands {
    fn run(&self, args: &Vec<String>) -> Option<&SystemCommand> {
        self.iter().find(|command| args.len() < 2 || command.triggers().contains(&args[1]))
    }
}