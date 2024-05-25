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
    Type,
}

impl ShellCommands {
    const VALUES: [Self; 3] = [Self::Exit, Self::Echo, Self::Type];
}

impl std::fmt::Display for ShellCommands {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ShellCommands::Exit => write!(f, "exit"),
            ShellCommands::Echo => write!(f, "echo"),
            ShellCommands::Type => write!(f, "type"),
        }
    }
}

fn pase_command_type(command: &str) -> ShellCommandType {
    // pase command
    let mut input_command_type = ShellCommandType::Unknow;
    for shell_command in ShellCommands::VALUES {
        if shell_command.to_string().eq(command) {
            input_command_type = ShellCommandType::Shell(shell_command);
            break;
        }
    }
    input_command_type
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

        // run command
        match pase_command_type(command_part) {
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
                ShellCommands::Type => {
                    let arg_command_type = pase_command_type(args_part);
                    match arg_command_type {
                        ShellCommandType::Unknow => {
                            println!("{}: command not found\\n", args_part);
                        }
                        ShellCommandType::Program => {
                            todo!();
                        }
                        ShellCommandType::Shell(_) => {
                            println!("{} is a shell builtin", args_part);
                        }
                    }
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
