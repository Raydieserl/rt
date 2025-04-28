use super::command_trait::CommandTrait;
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
        if let Some((_, cmd)) = self.get_idx_and_item(&args[1]) {
            cmd.run(args);
        }
    }

    fn remove_by_trigger(&mut self, args: &Vec<String>) {
        if let Some((i, _)) = self.get_idx_and_item(&args[2]) {
            self.remove(i);
        }
    }

    fn add_command(&mut self, args: &Vec<String>) {
        self.push(
            CustomCommand::new(
                vec![args[2].clone()],
                Some("Add description in commands.json".to_string()),
                vec![args[3].clone()],
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