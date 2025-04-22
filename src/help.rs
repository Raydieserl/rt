fn print_help(providers: Vec<Box<dyn HelpProviding>>) {
    for provider in providers {
        for item in provider.list_help_items() {
            item.print();
        }
    }
}

pub struct HelpItem {
    pub name: String,
    pub description: String
}

impl HelpItem {
    fn print(&self) {
        println!("{} {}", self.name, self.description)
    }
}

pub trait HelpProviding {
    fn list_help_items(&self) -> Vec<HelpItem>;
}