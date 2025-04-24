pub fn print_help(providers: Vec<Box<dyn HelpProviding>>) {
    println!("Usage: rt COMMAND [VARIABLES]");
    for provider in providers {
        println!("\n{}", provider.help_provider_title());
        for item in provider.list_help_items() {
            item.print();
        }
    }
}

pub struct HelpItemCMD {
    pub names: Vec<String>,
    pub description: String,
    pub variables: Vec<HelpItemVar>
}

pub struct HelpItemVar {
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

pub trait HelpProviding {
    fn help_provider_title(&self) -> String;
    fn list_help_items(&self) -> Vec<HelpItemCMD>;
}