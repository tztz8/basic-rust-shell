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
    if let ShellCommandType::Unknow = input_command_type {
        if std::path::Path::new(command).exists() {
            input_command_type = ShellCommandType::Program(String::from(command));
        }
    }
    if let ShellCommandType::Unknow = input_command_type {
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
            if let ShellCommandType::Unknow = input_command_type {
            } else {
                break;
            }
        }
    }
    input_command_type
}

fn absolute_path(path: &std::path::Path) -> std::io::Result<std::path::PathBuf> {
    let absolute_path = if path.is_absolute() {
        path.to_path_buf()
    } else {
        std::env::current_dir()?.join(path)
    };

    let mut out = Vec::new();

    use std::path::Component;

    for comp in absolute_path.components() {
        match comp {
            Component::CurDir => (),
            Component::ParentDir => match out.last() {
                Some(Component::RootDir) => (),
                Some(Component::Normal(_)) => {
                    out.pop();
                }
                None
                | Some(Component::CurDir)
                | Some(Component::ParentDir)
                | Some(Component::Prefix(_)) => out.push(comp),
            },
            comp => out.push(comp),
        }
    }

    let clean_path = if !out.is_empty() {
        out.iter().collect()
    } else {
        std::path::PathBuf::from(".")
    };

    Ok(clean_path)
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
                            let path = absolute_path(&(std::path::Path::new(&path))).unwrap();
                            println!("{} is {}", args_part, path.to_str().unwrap());
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
                let mut program = std::process::Command::new(path);
                program.envs(std::env::vars());
                if !args_part.is_empty() {
                    program.arg(args_part);
                }
                let program = program.spawn();
                if let Ok(mut program) = program {
                    match program.wait() {
                        Ok(exit_code) => {
                            if !exit_code.success() {
                                eprintln!("Exit Fail : {}", exit_code);
                            }
                        }
                        Err(e) => {
                            eprintln!("Program crash: {:?}", e);
                        }
                    }
                } else {
                    eprintln!("Program Fail to start: {:?}", program.err().unwrap());
                }
            }
        }
        std::io::stdout().flush().unwrap();
    }
}
