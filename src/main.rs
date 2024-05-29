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
    PrintWorkingDirectory,
    ChangeDirectory,
}

impl ShellCommands {
    const VALUES: [Self; 5] = [
        Self::Exit,
        Self::Echo,
        Self::Type,
        Self::PrintWorkingDirectory,
        Self::ChangeDirectory,
    ];
}

impl std::fmt::Display for ShellCommands {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ShellCommands::Exit => write!(f, "exit"),
            ShellCommands::Echo => write!(f, "echo"),
            ShellCommands::Type => write!(f, "type"),
            ShellCommands::PrintWorkingDirectory => write!(f, "pwd"),
            ShellCommands::ChangeDirectory => write!(f, "cd"),
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
            let command_path = path.join(command);
            if command_path.exists() {
                input_command_type =
                    ShellCommandType::Program(command_path.to_str().unwrap().into());
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
                ShellCommands::Exit => shell_exit(args_part),
                ShellCommands::Echo => println!("{}", args_part),
                ShellCommands::Type => shell_type_command(&paths, args_part),
                ShellCommands::PrintWorkingDirectory => shell_pwd_command(),
                ShellCommands::ChangeDirectory => shell_cd_command(args_part),
            },
            ShellCommandType::Unknow => println!("{}: command not found", command_part),
            ShellCommandType::Program(path) => shell_run_program(path, args_part),
        }
        std::io::stdout().flush().unwrap();
    }
}

fn shell_run_program(path: String, args_part: &str) {
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

fn shell_cd_command(args_part: &str) {
    let mut path = std::path::PathBuf::new();
    path.push(args_part);
    // may need to be joined to working dir
    if !path.exists() {
        path = std::env::current_dir().unwrap().join(args_part);
    }
    // exist
    if !path.exists() {
        eprintln!("cd: {}: No such file or directory", path.to_str().unwrap());
    } else {
        // update path
        let update_working_dir_result = std::env::set_current_dir(path);
        if update_working_dir_result.is_err() {
            eprintln!(
                "Error on changing directory : {:?}",
                update_working_dir_result.err().unwrap()
            );
        }
    }
}

fn shell_pwd_command() {
    let current_dir = std::env::current_dir();
    if current_dir.is_err() {
        eprintln!("Can't get current working directory: {:?}", current_dir.unwrap_err());
    } else {
        let current_dir = current_dir.unwrap();
        println!("{}", current_dir.to_str().unwrap_or("UNKNOWN"));
    }
}

fn shell_type_command(paths: &Vec<&std::path::Path>, args_part: &str) {
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


fn shell_exit(args_part: &str) {
    let is_args_emp = args_part.is_empty();
    let mut exit_code: i32 = args_part.parse().unwrap_or(-1);
    if is_args_emp {
        exit_code = 0;
    }
    std::process::exit(exit_code);
}