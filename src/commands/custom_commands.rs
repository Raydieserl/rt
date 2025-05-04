use super::command_trait::CommandTrait;
use super::custom_command::CustomCommand;
use super::exit::exit_status_one;


// CustomCommands
pub type CustomCommands = Vec<CustomCommand>;
pub trait CustomCommandsTrait {
    fn run(&self, trigger: &String, parameters: &Vec<&String>);
    fn remove_by_trigger(&mut self, trigger: &String);
    fn add_command(&mut self, trigger: &String, command: &String);
    fn get_idx_and_item(&self, trigger: &String) -> Option<(usize, &CustomCommand)>;
}

impl CustomCommandsTrait for CustomCommands {
    fn run(&self, trigger: &String, parameters: &Vec<&String>) {
        if let Some((_, cmd)) = self.get_idx_and_item(trigger) {
            cmd.run(parameters);
        }
    }

    fn remove_by_trigger(&mut self, trigger: &String) {
        if let Some((i, _)) = self.get_idx_and_item(trigger) {
            self.remove(i);
        }
    }

    fn add_command(&mut self, trigger: &String, command: &String) {
        self.push(
            CustomCommand::new(
                vec![trigger.clone()],
                Some("Add description in commands.json".to_string()),
                vec![command.clone()],
                None,
                None
            )
        );
    }

    fn get_idx_and_item(&self, trigger: &String) -> Option<(usize, &CustomCommand)> {
        let result = self.iter().enumerate().find(
            |(_, cmd)| cmd.triggers().contains(&trigger)
        );
        if let Some(_) = result {
            return result;
        }
        exit_status_one(&format!("Custom command not found: {}", trigger));
        return None;
    }
}