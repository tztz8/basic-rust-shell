use std::io::Write;

/// # Start of the shell program
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
        match parse_command_type(&paths, command_part) {
            ShellCommandType::Shell(shell_command) => match shell_command {
                ShellCommands::Exit => shell_exit(args_part),
                ShellCommands::Echo => println!("{}", args_part),
                ShellCommands::Type => shell_type_command(&paths, args_part),
                ShellCommands::PrintWorkingDirectory => shell_pwd_command(),
                ShellCommands::ChangeDirectory => shell_cd_command(args_part),
            },
            ShellCommandType::Unknown => println!("{}: command not found", command_part),
            ShellCommandType::Program(path) => shell_run_program(path, args_part),
        }
        std::io::stdout().flush().unwrap();
    }
}

/// # Shell Command Type
///
/// Enum of what type of command
enum ShellCommandType {
    Shell(ShellCommands),
    Program(String),
    Unknown,
}

/// # Shell Built In Commands
///
/// Enum of all shell's built in commands
#[derive(Debug)]
enum ShellCommands {
    Exit,
    Echo,
    Type,
    PrintWorkingDirectory,
    ChangeDirectory,
}

// Java enum VALUES
impl ShellCommands {
    const VALUES: [Self; 5] = [
        Self::Exit,
        Self::Echo,
        Self::Type,
        Self::PrintWorkingDirectory,
        Self::ChangeDirectory,
    ];
}

// to_string
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

/// # Get Command Type
///
/// parse the command to get the type of command
fn parse_command_type(paths: &Vec<&std::path::Path>, command: &str) -> ShellCommandType {
    // pase command
    let mut input_command_type = ShellCommandType::Unknown;
    // is shell command?
    for shell_command in ShellCommands::VALUES {
        if shell_command.to_string().eq(command) {
            input_command_type = ShellCommandType::Shell(shell_command);
            break;
        }
    }
    // is relative/absolute program command?
    if let ShellCommandType::Unknown = input_command_type {
        // is home relative program command?
        let command = home_folder_path(command);
        if std::path::Path::new(command.as_str()).exists() {
            input_command_type = ShellCommandType::Program(command);
        }
    }
    // is path program command?
    if let ShellCommandType::Unknown = input_command_type {
        for path in paths {
            let command_path = path.join(command);
            if command_path.exists() {
                input_command_type =
                    ShellCommandType::Program(command_path.to_str().unwrap().into());
                break;
            }
        }
    }
    // return type
    input_command_type
}

// ################
// # Path Helpers #
// ################

/// # absolute path
///
/// Convert a path to an absolute path
fn absolute_path(path: &std::path::Path) -> std::io::Result<std::path::PathBuf> {
    // Make a valid path
    let absolute_path = if path.is_absolute() {
        path.to_path_buf()
    } else {
        std::env::current_dir()?.join(path)
    };

    // Clean path
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

    // Is there any path left
    let clean_path = if !out.is_empty() {
        out.iter().collect()
    } else {
        // This will **never** be that case as we are getting the absolute path
        std::path::PathBuf::from(".")
    };

    // Return the cleanup absolute path
    Ok(clean_path)
}

/// # Update path string
///
/// if path string starts with ~ it is replaced with home folder, otherwise return what given
fn home_folder_path(path: &str) -> String {
    if path.starts_with('~') {
        let mut new_path = String::from(path);
        new_path.remove(0);
        if let Ok(home_path) = std::env::var("HOME") {
            new_path.insert_str(0, home_path.as_str());
        } else {
            eprintln!("Env Missing Home folder");
        }
        return new_path;
    }
    return String::from(path);
}

// #################
// # Shell Helpers #
// #################

/// # Shell run program
///
/// Run the external program and hand any problems.
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

/// # Shell Change Directory Command
///
/// Change the current working directory of this program (shell)
fn shell_cd_command(path_str: &str) {
    let path_str = home_folder_path(path_str);
    let mut path = std::path::PathBuf::new();
    path.push(path_str.as_str());
    // may need to be joined to working dir
    if !path.exists() {
        path = std::env::current_dir().unwrap().join(path_str.as_str());
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

/// # Shell Command Type
///
/// The `type` builtin is used to determine how a command would be interpreted if used.
/// **(for the user)**
fn shell_type_command(paths: &Vec<&std::path::Path>, args_part: &str) {
    let arg_command_type = parse_command_type(&paths, args_part);
    match arg_command_type {
        ShellCommandType::Unknown => {
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

/// # Shell Command Exit
///
/// cause the shell to exit
///
/// > The `exit` utility shall cause the shell to exit from its current execution environment
/// > with the exit status specified by the unsigned decimal integer n. If the current execution
/// > environment is a subshell environment, the shell shall exit from the subshell environment
/// > with the specified exit status and continue in the environment from which that subshell
/// > environment was invoked; otherwise, the shell utility shall terminate with the specified
/// > exit status. If n is specified, but its value is not between `0` and `255` inclusively,
/// > the exit status is undefined.
/// **From** [Open Group IEEE](https://pubs.opengroup.org/onlinepubs/9699919799/utilities/V3_chap02.html#exit)
fn shell_exit(exit_code_str: &str) {
    let is_args_emp = exit_code_str.is_empty();
    let mut exit_code: i32 = exit_code_str.parse().unwrap_or(-1);
    if is_args_emp {
        exit_code = 0;
    }
    std::process::exit(exit_code);
}