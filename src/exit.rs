use std::process;

pub fn exit_status_one(message: &String) {
    eprintln!("{message}");
    process::exit(1);
}