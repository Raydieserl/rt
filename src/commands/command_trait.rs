use super::command_variables::CommandVariable;

pub trait CommandTrait {
    fn triggers(&self) -> &Vec<String>;
    fn description(&self) -> Option<&String>;
    fn variables(&self) -> Option<&Vec<CommandVariable>>;
    fn groups(&self) -> Option<&Vec<String>>;
}