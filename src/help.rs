pub fn print_help(providers: Vec<Box<dyn HelpProviding>>) {
    println!("Usage: rt [OPTIONS] COMMAND");
    for provider in providers {
        println!("\n{}", provider.help_provider_title());
        for item in provider.list_help_items() {
            item.print();
        }
    }
}

pub struct HelpItem {
    pub names: Vec<String>,
    pub description: String
}

impl HelpItem {
    fn print(&self) {
        println!("   {}: {}", self.names.join(", "), self.description)
    }
}

pub trait HelpProviding {
    fn help_provider_title(&self) -> String;
    fn list_help_items(&self) -> Vec<HelpItem>;
}