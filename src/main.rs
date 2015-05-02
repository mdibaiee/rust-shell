extern crate rust_shell as shell;

use shell::commands;
use std::io;
use std::io::Write;
use std::process::Command;
use std::str;

fn main() {
    println!("Welcome to Rust Shell, try |help|");

    let mut directory: String = "/".to_string();

    loop {
        let (cmd, args) = read(&directory);

        match cmd.as_ref() {
            "help" => commands::help::exec(args),
            "cd" => directory = commands::cd::exec(&directory, args),
            "ls" => commands::ls::exec(&directory, args),
            "echo" => commands::echo::exec(args),
            _ => run_command(&directory, cmd, args)
        };
    }
}

fn read(directory: &str) -> (String, Vec<String>) {
    print!("{} ~ $ ", directory);
    let mut stdout = io::stdout();
    // Flush to ensure stdout is printed immediately
    stdout.flush().unwrap();

    let mut stdin = io::stdin();
    let mut line: String = "".to_string();

    stdin.read_line(&mut line).unwrap();

    // Last character is a line-break we don't need
    line.pop();

    let params: Vec<String> = line.split(" ").map(|x| x.to_string()).collect();
    let mut iter = params.into_iter();

    let cmd = iter.next().unwrap();
    let rest: Vec<String> = iter.collect();

    (cmd, rest)
}

fn run_command(directory: &str, command: String, args: Vec<String>) -> () {
    let out = match Command::new(command)
                            .args(&args)
                            .current_dir(directory)
                            .output() {
        Ok(out) => out.stdout,
        Err(e) => {
            println!("Error: {}", e);
            vec![0u8]
        }
    };

    println!("{}", str::from_utf8(&out).unwrap());
}
