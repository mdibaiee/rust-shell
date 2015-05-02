use super::option_parser::parse;
use std::str::FromStr;

pub fn exec(args: Vec<String>) {
    let (options, mut params) = parse(args);

    for param in params.iter_mut() {
        param.push_str("\n");
    }

    let mut options_iter = options.iter();
    let break_option = options_iter.position(|x| *x == "break".to_string());

    let add_break: u32 = match break_option {
        Some(..) => u32::from_str(options_iter.next().unwrap()).unwrap(),
        None => 0u32
    };

    for i in 0..add_break {
        println!("");
    };

    let message: String = params.iter().map(|x| x.as_ref()).collect();

    println!("{}", message);
}

pub fn help() {
    println!(" Prints out stuff\n
 usage: echo <line1> <line2> ... <lineN>\n\n

 Options:\n
    --break=n   Adds |n| line breaks before message\n");
}
