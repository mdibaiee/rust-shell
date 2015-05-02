use super::{cd, ls, echo};

pub fn exec(args: Vec<String>) {
    if args.len() > 0 {

        match args[0].as_ref() {
            "cd" => cd::help(),
            "ls" => ls::help(),
            "echo" => echo::help(),
            _ => println!("Command not found")
        }

    } else {
        println!("
 I wrote this program to learn Rust.\n\n
 use help <command> for info about an specific command.\n\n

 The shell supports only two kinds of arguments:\n
    Options:\n
        Single-dash options with no value e.g |ls -a|\n
        Double-dash options in key=value format, spaces not allowed |echo --break=2|\n
    Parameters:\n
        Anything that's not an option, is a parameter\n\n

 Complex things like pipes are not implemented.\n
 Commands:\n
    cd, ls, pwd\n");
    }
}
