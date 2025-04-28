use crate::commands::custom_commands::CustomCommands;

pub fn serialize(custom_commands: &CustomCommands) -> String {
    serde_json::to_string(&custom_commands).unwrap()
}

pub fn deserialize(custom_commands: &String, fallback_commands: &String) -> CustomCommands {
    serde_json::from_str(&custom_commands)
        .unwrap_or_else(|error| {
            print_parse_error(&error.to_string());
            serde_json::from_str(&fallback_commands).unwrap()
        })
}

fn print_parse_error(error: &String) {
    let sep = "!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!";
    println!("\n{sep}");
    println!(
        "Parsing of commands.json failed\nError: {}\nFix: Try importing a backup or fix the error manually.", 
        error
    );
    println!("{sep}\n");
}