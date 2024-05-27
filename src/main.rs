use std::io::Write;

enum ShellCommandType {
    Shell(ShellCommands),
    Program(String),
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

fn pase_command_type(paths: &Vec<&std::path::Path>, command: &str) -> ShellCommandType {
    // pase command
    let mut input_command_type = ShellCommandType::Unknow;
    for shell_command in ShellCommands::VALUES {
        if shell_command.to_string().eq(command) {
            input_command_type = ShellCommandType::Shell(shell_command);
            break;
        }
    }
    match input_command_type {
        ShellCommandType::Unknow => {
            for path in paths {
                match std::fs::read_dir(path) {
                    Ok(entries) => {
                        for entry in entries {
                            match entry {
                                Ok(entry) => {
                                    if entry.file_name().eq(command) {
                                        input_command_type = ShellCommandType::Program(
                                            entry.path().to_str().unwrap().into(),
                                        );
                                        break;
                                    }
                                }
                                Err(e) => {
                                    eprintln!("Path Command Pase - entries - Error: {}", e);
                                }
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Path Command Pase - Read Dir - Error: {}", e);
                    }
                }
                match input_command_type {
                    ShellCommandType::Unknow => {}
                    _ => {
                        break;
                    }
                }
            }
        }
        _ => {}
    }
    input_command_type
}

fn main() {
    // path
    let env_path = std::env::var("PATH").unwrap_or(String::from(""));
    let paths_str = env_path.split(':');
    let mut paths = Vec::new();
    for path_str in paths_str {
        paths.push(std::path::Path::new(path_str));
    }
    let paths = paths;
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
        match pase_command_type(&paths, command_part) {
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
                    let arg_command_type = pase_command_type(&paths, args_part);
                    match arg_command_type {
                        ShellCommandType::Unknow => {
                            println!("{}: not found", args_part);
                        }
                        ShellCommandType::Program(path) => {
                            println!("{} is {}", args_part, path);
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
            ShellCommandType::Program(path) => {
                println!("{}", path);
            }
        }
        std::io::stdout().flush().unwrap();
    }
}
