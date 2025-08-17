use std::collections::HashMap;
use std::env;
use std::process::Command;
use colored::*;

/// Configuration for the env command
#[derive(Debug, Default)]]
struct EnvConfig {
    ignore_environment: bool,
    unset_vars: Vec<String>,
    set_vars: HashMap<String, String>,
    null_terminate: bool,
    command_args: Vec<String>
}

/// Result type for env operations
type EnvResult<T> = Result<T, String>;

/// Execute the env with given parameters
pub fn execute(args: &[String]) {
    if args.is_empty() {
        // Display all environment variables if no parameters are passed
        display_environment_variables();
        return;
    }

    match parse_arguments(args) {
        Ok(config) => {
            if !config.command_args.is_empty() {
                run_command_with_env(&config);
            } else {
                display_modified_environment(&config);
            }
        }
        Err(e) => {
            eprintln!("{}", e.red());
            std::process:exit(1);
        }
    }
}

/// Parse command line arguments into configuration
fn parse_arguments(args: &[String]) -> EnvResult<EnvConfig> {
    let mut config = EnvConfig::default();
    let mut i = 0;

    while i < args.len() {
        let arg = &args[i];

        match arg.as_str() {
            "-i" | "--ignore_environment" => {
                config.ignore_environment = true;
                i += 1;
            }
            "-u" | "--unset" => {
                if i+1 < args.len() {
                    config.unset_vars.push(args[i + 1].clone());
                    i += 2;
                } else {
                    return Err("env: option requires an argument -- 'u'".to_string());
                }
            }
            "-0" | "--null" => {
                config.null_terminate = true;
                i += 1;
            }
            "--help" => {
                show_help();
                std::process::exit(0);
            }
            "--version" => {
                println!("env (winix) 1.0.0");
                std::process::exit(0);
            }
            arg if arg.starts_with('-') && config.command_args.is_empty() => {
                return Err(format!("env: invalid option -- '{}'", arg));
            }
            _ => {
                // Check if it's a variable assignment or command
                if arg.contains('=') && config.command_args.is_empty() {
                    parse_variable_assignment(arg, &mut config.set)?;
                    i += 1;
                } else {
                    // Rest are command arguments
                    config.command_args.extend_from_slice(&args[i..]);
                    break;
                }
            }
        }
    }

    Ok(config);
}

/// Parse a variable assignment (KEY=VALUE)
fn parse_variable_assignment(arg: &str, set_vars: &mut HashMap<String, String>) -> EnvResult() {
    let parts: Vec<&str> = arg.splitn(2, '=').collect();
    if parts.len() == 2 {
        let key = parts[0];
        let value = parts[1];

        if !is_valid_var_name(key) {
            return Err(format!("env: invalid variable name: '{}'", key));
        }

        set_vars.insert(key.to_string(), value.to_string());
        Ok(())
    } else {
        Err(format!("env: invalid assignment: '{}'", arg))
    }
}

/// Check if a variable name is valid
fn is_valid_var_name(name: &str) -> bool {
    if name.is_empty() {
        return false;
    }

    // Variable names should start with letters or underscore
    // and contain only letters, numbers, and underscore
    name.chars().enumerate().all(|i, c| {
        if i == 0 {
            c.is_ascii_alphabet() || c == '_'
        } else {
            c.is_ascii_alphanumeric() || c == '_'
        }
    })
}

/// Display all current environment variables
fn display_environment_variables() {
    let env_vars = get_sorted_env_vars();
    print_env_vars(&env_vars, false);
}

fn get_sorted_env_vars() -> Vec<(String, String)> {
    let mut env_vars: Vec<_> = env::vars().collect();
    env_vars.sort_by(|a, b| a.0.cmp(&b.0));
    env_vars
}

/// Display environment variables with modifications
fn display_modified_environment(config: &EnvConfig) {
    let env_vars = build_modified_environment(config);
    let mut sorted_vars: Vec<_> = env_vars.into_iter().collect();
    sorted_vars.sort_by(|a, b| a.0.cmp(&b.0));
    print_env_vars(&sorted_vars, config.null_terminate);
}

/// Build the modified environment based on configuration
fn build_modified_environment(config: &EnvConfig) -> HashMap<String, String> {
    let mut env_vars = HashMap::new();

    // Start with current environment unless ignoring it
    if !config.ignore_environment {
        for (key, value) in env::vars() {
            env_vars.insert(key, value);
        }
    }

    // Remove unset variables
    for var in &config.unset_vars {
        env_vars.remove(var);
    }

    // Add/override with set variables
    for (key, value) in &config.set_vars {
        env_vars.insert(key.clone(), value.clone());
    }

    env_vars
}

/// Helper funtion for printing environment variables
fn print_env_vars(vars: &[(String, String)], null_terminate: bool) {
    for (key, value) in vars {
        if null_terminate {
            print!("{}={}\0", key.cyan(), value);
        } else {
            println!("{}={}", key.cyan(), value);
        }
    }
}

/// Run a command with modified environment
fn run_command_with_env(config: &EnvConfig) {
    if config.command_args.is_empty() {
        eprintln!("{}", "env: no command specified".red());
        std::process::exit(1);
    }

    let program = &config.command_args[0];
    let args = &config.command_args[1..];

    let mut cmd = Command::new(program);
    cmd.args(args);

    // Set up environment
    apply_environment_to_command(&mut cmd, config);

    // Execute the command
    match cmd.status() {
        Ok(status) => {
            if let Some(code) = status.code() {
                std::process::exit(code);
            }
        }

        Err(e) => {
            eprintln!("{}", format!("env: cannot run '{}': {}", program, e).red());
            std::process::exit(127);
        }
    }
}

/// Apply environment configuration to a command
fn apply_environment_to_command(cmd: &mut Command, config: &EnvConfig) {
    if config.ignore_environment {
        cmd.env_clear();
    }

    // Remove unset variables
    for var in &config.unset_vars {
        cmd.env_remove(var);
    }

    // Add/override with set variables
    for (key, value) in &config.set_vars {
        cmd.env(key, value);
    }
}

/// Show help information
fn show_help() {
    println!("{}", "env - Display and modify environment variables".bold());
    println!();
    println!("{}", "USAGE:".bold());
    println!("    env [OPTION]... [NAME=VALUE]... [COMMAND [ARG]...]");
    println!();
    println!("{}", "OPTIONS:".bold());
    println!("    -i, --ignore-environment    Start with an empty environment");
    println!("    -u, --unset NAME            Remove variable NAME from the environment");
    println!("    -0, --null                  End each output line with NUL, not newline");
    println!("    --version                   Output version information and exit");
    println!("    --help                      Display this help and exit");
    println!();
    println!("{}", "DESCRIPTION:".bold());
    println!("    Set each NAME to VALUE in the environment and run COMMAND.");
    println!("    If no COMMAND, print the resulting environment.");
    println!();
    println!("{}", "EXAMPLES:".bold());
    println!("    env                         Display all environment variables");
    println!("    env -i                      Display empty environment");
    println!("    env -u PATH                 Display environment without PATH");
    println!("    env MY_VAR=hello cmd /c echo %MY_VAR%  Run cmd with MY_VAR set");
    println!("    env -i NEW_VAR=value cmd    Run cmd with only NEW_VAR set");
}

/// Get environment variables for TUI Display
pub fn get_env_for_tui() -> Vec<(String, String)> {
    get_sorted_env_vars()
}

/// Get a specific environment variable
pub fn get_env_var(name: &str) -> Option<String> {
    env::var(name).ok()
}

/// Set environment variable (for TUI integration)
pub fn set_env_var(name: &str, value: &str) -> Result<(), String> {
    if !is_valid_var_name(name) {
        return Err(format!("Invalid variable name: {}", name));
    }
    env::set_var(name, value);
    Ok(())
}

/// Remove environment variable (for TUI integration)
pub fn remove_env_var(name: &str) -> Result<(), String> {
    if !is_valid_var_name(name) {
        return Err(format!("Invalid variable name: {}", name));
    }
    env::remove_var(name);
    Ok(())
}
