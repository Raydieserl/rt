use super::custom_command::CustomCommand;
use super::exit::exit_status_one;


// CustomCommands
pub type CustomCommands = Vec<CustomCommand>;
pub trait CustomCommandsTrait {
    fn run(&self, args: &Vec<String>);
    fn remove_by_trigger(&mut self, args: &Vec<String>);
    fn add_command(&mut self, args: &Vec<String>);
    fn get_idx_and_item(&self, trigger: &String) -> Option<(usize, &CustomCommand)>;
}

impl CustomCommandsTrait for CustomCommands {
    fn run(&self, args: &Vec<String>) {
        if let Some((_, ccommand)) = self.get_idx_and_item(&args[1]) {
            ccommand.run(args);
        }
    }

    fn remove_by_trigger(&mut self, args: &Vec<String>) {
        if let Some((i, _)) = self.get_idx_and_item(&args[2]) {
            self.remove(i);
        }
    }

    fn add_command(&mut self, args: &Vec<String>) {
        self.push(
            CustomCommand {
                triggers: vec![args[2].clone()],
                description: "Add description in commands.json".to_string(),
                commands: vec![args[3].clone()],
                variables: vec![]
            }
        );
    }

    fn get_idx_and_item(&self, trigger: &String) -> Option<(usize, &CustomCommand)> {
        let result = self.iter().enumerate().find(|(_, ccommand)| ccommand.triggers.contains(&trigger));
        if let Some(_) = result {
            return result;
        }
        exit_status_one(&format!("Custom command not found: {}", trigger));
        return None;
    }
}