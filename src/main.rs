pub mod storage;
pub mod print;
mod commands;

use std::{ env, process };
use commands::*;
use print::*;
use storage::*;

fn main() {
    let args = get_args();
    let command = get_command(&args);
    let storage = StorageTxt::new("storage.txt".to_string());
    let mut printer = StdoutPrint;

    let exit_code = match command.as_str() {
        "add" => AddCommand::new(args).handle(&storage, &mut printer),
        "list" => ListCommand::new().handle(&storage, &mut printer),
        _ => {
            println!("Unknown command");

            1
        }
    };

    process::exit(exit_code);
}

fn display_help() -> () {
    println!("Usage: todo <command> <args>");
    println!();
    println!("Commands:");
    println!("  add <description> - adds a todo");
    println!("  list              - displays all todos");
    println!();
}

fn get_args() -> Vec<String> {
    let args: Vec<String> = env::args().collect();

    args
}

fn get_command(args: &Vec<String>) -> String {
    let command = args.get(1).unwrap_or_else(|| {
        display_help();

        process::exit(0);
    });

    command.to_string()
}
