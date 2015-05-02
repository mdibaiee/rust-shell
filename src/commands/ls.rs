use std::path::Path;
use std::fs;
use std::fs::File;
use super::option_parser::parse;

pub fn exec(directory: &str, args: Vec<String>) {
    let (options, params) = parse(args);

    let dir: &str = if params.len() > 0 {
        &params[0]
    } else {
        directory
    };

    let dir_path = Path::new(dir);
    let dir_entry = fs::read_dir(dir_path).unwrap().map(|x| x.unwrap());
    let details = options.contains(&"l".to_string());

    if !&options.contains(&"a".to_string()) {
        let filtered = dir_entry.filter_map(|entry| {
            let path = entry.path();
            let is_hidden = path.file_name().unwrap().to_str().unwrap().starts_with(".");

            if is_hidden {
                None
            } else {
                Some(entry)
            }
        });

        print_files(filtered, details);
    } else {
        print_files(dir_entry, details);
    };
}

fn print_files<T: Iterator<Item=fs::DirEntry>>(entries: T, detailed: bool) {

    if detailed {
        println!("Name\tSize");
    };

    for entry in entries {
        let path = entry.path();
        let file_name = path.file_name().unwrap().to_str().unwrap();

        if !detailed {
            println!("{}", file_name);
            continue;
        };

        let file = File::open(&path).unwrap();
        let meta = file.metadata().unwrap();
        let size = meta.len();

        println!("{}\t{}", file_name, size);
    };
}

pub fn help() {
    println!(" List directory contents\n
 usage: ls <directory>\n
 If no directory is specified, the current directory's files are listed\n\n
 Options:\n
   -a    Show hidden files
   -l    Show details\n");
}
