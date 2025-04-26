use crate::cmd_custom::CustomCMD;
use crate::exit::exit_status_one;
use crate::help::{HelpItem, HelpItemCMDProviding, HelpProviding};

// CustomCMDs
pub type CustomCMDs = Vec<CustomCMD>;
pub trait CustomCMDsTrait {
    fn run(&self, args: &Vec<String>);
    fn remove_by_trigger(&mut self, args: &Vec<String>);
    fn get_idx_and_item(&self, trigger: &String) -> Option<(usize, &CustomCMD)>;
}

impl CustomCMDsTrait for CustomCMDs {
    fn run(&self, args: &Vec<String>) {
        if let Some((_, ccmd)) = self.get_idx_and_item(&args[1]) {
            ccmd.run(args);
        }
    }

    fn remove_by_trigger(&mut self, args: &Vec<String>) {
        if let Some((i, _)) = self.get_idx_and_item(&args[2]) {
            self.remove(i);
        }
    }

    fn get_idx_and_item(&self, trigger: &String) -> Option<(usize, &CustomCMD)> {
        let result = self.iter().enumerate().find(|(_, ccmd)| ccmd.triggers.contains(&trigger));
        if let Some(_) = result {
            return result;
        }
        exit_status_one(&format!("Custom command not found: {}", trigger));
        return None;
    }
}

impl HelpProviding for CustomCMDs {
    fn help_item(&self) -> crate::help::HelpItem {
        HelpItem {
            title: "Custom Commands: ".to_string(),
            commands: self.iter().map(|e|e.help_item_cmd()).collect()
        }
    }
}