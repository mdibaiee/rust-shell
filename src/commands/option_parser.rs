pub fn parse(args: Vec<String>) -> (Vec<String>, Vec<String>) {
    let iter = args.into_iter();
    let mut options: Vec<String> = vec![];
    let mut parameters: Vec<String> = vec![];

    for item in iter {
        let i: &str = item.as_ref();
        if i.starts_with("--") {

            let split: Vec<String> = i.split("=").map(|x| x.to_string()).collect();
            let key = split[0].chars().skip(2).collect::<String>();
            let value = split[1].to_string();
            options.push(key);
            options.push(value);
        } else if i.starts_with("-") {
            let chars = i.chars().skip(1);
            let string: String = chars.collect::<String>();
            options.push(string.to_string());
            options.push("".to_string());
        } else {

            parameters.push(item.to_string());
        }
    };

    (options, parameters)
}
