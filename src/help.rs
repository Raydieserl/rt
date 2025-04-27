pub fn print_help(help_items: Vec<HelpItem>) {
    println!("Usage: rt COMMAND [VARIABLES]");
    for hi in help_items {
        println!("\n{}", hi.title);
        for command in &hi.commands { command.print(); }
    }
}
pub struct HelpItem {
    pub title: String,
    pub commands: Vec<HelpItemCommand>
}

pub struct HelpItemCommand {
    pub triggers: Vec<String>,
    pub description: String,
    pub variables: Vec<HelpItemCommandVar>
}

pub struct HelpItemCommandVar {
    pub name: String,
    pub description: String
}

impl HelpItemCommand {
    fn print(&self) {
        println!("   {}: {}", self.triggers.join(", "), self.description);
        for var in &self.variables {
            println!("      {}: {}", var.name, var.description);
        }
    }
}

pub trait HelpProviding { fn help_item(&self) -> HelpItem; }
pub trait HelpItemCommandProviding { fn help_item_command(&self) -> HelpItemCommand; }