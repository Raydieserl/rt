pub fn print_help(help_items: Vec<HelpItem>) {
    println!("Usage: rt COMMAND [VARIABLES]");
    for hi in help_items {
        println!("\n{}", hi.title);
        for cmd in &hi.commands { cmd.print(); }
    }
}
pub struct HelpItem {
    pub title: String,
    pub commands: Vec<HelpItemCMD>
}

pub struct HelpItemCMD {
    pub names: Vec<String>,
    pub description: String,
    pub variables: Vec<HelpItemCMDVar>
}

pub struct HelpItemCMDVar {
    pub name: String,
    pub description: String
}

impl HelpItemCMD {
    fn print(&self) {
        println!("   {}: {}", self.names.join(", "), self.description);
        for var in &self.variables {
            println!("      {}: {}", var.name, var.description);
        }
    }
}

pub trait HelpProviding { fn help_item(&self) -> HelpItem; }
pub trait HelpItemCMDProviding { fn help_item_cmd(&self) -> HelpItemCMD; }