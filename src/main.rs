use std::io::Write;

#[allow(dead_code)]
enum ShellCommandType {
    Shell(ShellCommands),
    Program,
    Unknow,
}

#[derive(Debug)]
enum ShellCommands {
    Exit,
    Echo,
}

impl ShellCommands {
    const VALUES: [Self; 2] = [Self::Exit, Self::Echo];
}

impl std::fmt::Display for ShellCommands {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ShellCommands::Exit => write!(f, "exit"),
            ShellCommands::Echo => write!(f, "echo"),
        }
    }
}

fn main() {
    loop {
        // Ask for user input
        print!("$ ");
        std::io::stdout().flush().unwrap();

        // Wait for user input
        let stdin = std::io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        // parse user input to command and args
        let input = input.trim();
        let end_command_index = input.find(' ').unwrap_or(input.len());
        let (command_part, args_part) = input.split_at(end_command_index);
        let args_part = args_part.trim();

        // pase command
        let mut input_command_type = ShellCommandType::Unknow;
        for shell_command in ShellCommands::VALUES {
            if shell_command.to_string().eq(command_part) {
                input_command_type = ShellCommandType::Shell(shell_command);
                break;
            }
        }

        // run command
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
                ShellCommands::Echo => {
                    println!("{}", args_part);
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
