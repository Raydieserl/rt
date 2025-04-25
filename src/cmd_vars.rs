use serde::{Serialize, Deserialize};

use crate::{exit::exit_status_one, help::HelpItemCMDVar};


type CMDVariables = Vec<CMDVariable>;
pub trait CMDVariablesTrait {
    fn as_help_item_cmd_vars(&self) -> Vec<HelpItemCMDVar>;
    fn exit_if_vars_do_not_match(&self, args: &Vec<String>);
}

impl CMDVariablesTrait for CMDVariables {
    fn as_help_item_cmd_vars(&self) -> Vec<HelpItemCMDVar> {
        self.iter().map(
            |v| HelpItemCMDVar { 
                name: v.target.clone(), 
                description: v.description.clone() 
            }
        ).collect()
    }

    fn exit_if_vars_do_not_match(&self, args: &Vec<String>) {
        if self.len() > 0  && args.len() < self.len() + 2 {
            let args: Vec<String> = self.iter().map(|e| e.target.clone()).collect();
            exit_status_one(&format!("Missing arguments for command: {}", args.join(", ")));
        }
    }
}

// CMD Var
#[derive(Serialize, Deserialize, Debug)]
pub struct CMDVariable {
    pub description: String,
    pub target: String
}