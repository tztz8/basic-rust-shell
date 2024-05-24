use std::io::{self, Write};

#[allow(dead_code)]
enum ShellCommandType {
    Shell(ShellCommands),
    Program,
    Unknow,
}

#[derive(Debug)]
enum ShellCommands {
    Exit,
}

impl ShellCommands {
    const VALUES: [Self; 1] = [Self::Exit];
}

impl std::fmt::Display for ShellCommands {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ShellCommands::Exit => write!(f, "exit"),
        }
    }
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
        let (command_part, args_part) = input.split_at(end_command_index);
        let args_part = args_part.trim();

        let mut input_command_type = ShellCommandType::Unknow;

        for shell_command in ShellCommands::VALUES {
            if shell_command.to_string().eq(command_part) {
                input_command_type = ShellCommandType::Shell(shell_command);
                break;
            }
        }

        match input_command_type {
            ShellCommandType::Shell(shell_command) => match shell_command {
                ShellCommands::Exit => {
                    let is_args_emp = args_part.is_empty();
                    let mut exit_code: i32 = args_part.parse().unwrap_or(-1);
                    if is_args_emp {
                        exit_code = 0;
                    }
                    std::process::exit(exit_code);
                }
            },
            ShellCommandType::Unknow => {
                println!("{}: command not found", command_part);
            }
            ShellCommandType::Program => {
                todo!();
            }
        }
    }
}
