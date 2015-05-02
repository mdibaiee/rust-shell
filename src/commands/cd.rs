use std::path::Path;
use std::fs;

pub fn exec(directory: &str, args: Vec<String>) -> String {
    let dir = args[0].to_string();
    let current_path = Path::new(directory);

    // Match . and .. and return early if needed
    match dir.as_ref() {
        "." => return directory.to_string(),
        ".." => return current_path.parent().unwrap().to_str().unwrap().to_string(),
        _ => ()
    };

    let new_path = Path::new::<String>(&dir);

    let joined_path = current_path.join(new_path);
    let path = joined_path.as_path();

    let path_string = match path.to_str() {
        Some(value) => value.to_string(),
        None => directory.to_string()
    };

    if !check_folder(&path) {
        println!("Directory not found {}", path_string);
        directory.to_string()
    } else {
        path_string
    }
}

fn check_folder(directory: &Path) -> bool {
    match fs::read_dir(directory) {
        Ok(..) => true,
        Err(..) => false
    }
}

pub fn help() {
    println!(" Change Directory\n
 usage: cd <directory>\n

 special:
    .. => parent
    . => current");
}
