use std::io::{self, Write};

#[allow(dead_code)]
enum ShellCommandType {
    Shell,
    Program,
    Unknow,
}

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        let input = input.trim();
        let end_command_index = input.find(' ').unwrap_or(input.len());
        let (command_part, _args_part) = input.split_at(end_command_index);

        let input_command_type = ShellCommandType::Unknow;

        match input_command_type {
            ShellCommandType::Shell => {
                todo!();
            }
            ShellCommandType::Unknow => {
                println!("{}: command not found", command_part);
            }
            ShellCommandType::Program => {
                todo!();
            }
        }
    }
}
