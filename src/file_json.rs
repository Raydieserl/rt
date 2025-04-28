use crate::commands::{command_trait::CommandTrait, custom_commands::CustomCommands, system_command::SystemCommand};

pub fn serialize(custom_commands: &CustomCommands) -> String {
    serde_json::to_string(&custom_commands).unwrap()
}

pub fn deserialize(args: &Vec<String>, custom_commands: &String, fallback_commands: &String) -> CustomCommands {
    serde_json::from_str(&custom_commands)
        .unwrap_or_else(|error| {
            print_parse_error(&args, &error.to_string());
            serde_json::from_str(&fallback_commands).unwrap()
        })
}

fn print_parse_error(args: &Vec<String>, error: &String) {
    if let Some(str) = args.get(1) { 
        if SystemCommand::Import.triggers().contains(str) { return }
    }
    let sep = "!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!";
    println!("\n{sep}");
    println!(
        "Parsing of commands.json failed\nError: {}\nFix: Try importing a backup or fix the error manually.", 
        error
    );
    println!("{sep}\n");
}