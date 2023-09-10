use std::fs::{read_to_string, write};


pub fn toggle_comment(args: &Vec<String>) {
    let filepath = args.get(2).expect("No filepath given.\n");
    let file_content = match read_to_string(filepath) {
        Ok(content) => content,
        Err(_) => {
            print!("File could not be read.\n");
            std::process::exit(1);
        }
    };

    let mut comment_prefix = String::from(args.get(3).expect("No comment prefix given.\n"));
    comment_prefix.push(' '); // an added space is assumed to be desired, maybe a flag one day
    let prefix_length = comment_prefix.len();

    if args.len() < 5 {
        print!("No line numbers given.\n");
        std::process::exit(1);
    }

    let line_numbers: &Vec<i32> = &args[4..]
        .iter()
        .map(|x| x.parse::<i32>().unwrap_or(-1))
        .collect();


    let mut out_string: String = "".to_string();
    let mut i: i32 = 0;
    for line in file_content.lines() {
        i += 1;

        if line_numbers.contains(&i) {
            if &line[0..prefix_length] == comment_prefix {
                out_string.push_str(&line[prefix_length..]);
            } else {
                out_string.push_str(&comment_prefix);
                out_string.push_str(&line);
            }
        } else {
            out_string.push_str(&line);
        }
        out_string.push_str("\n");
    }
    write(filepath, &out_string).expect("Unable to write file");
}

pub fn toggle_bool(args: &Vec<String>) {
    let filepath = args.get(2).expect("No filepath given.\n");
    let file_content = match read_to_string(filepath) {
        Ok(content) => content,
        Err(_) => {
            print!("File could not be read.\n");
            std::process::exit(1);
        }
    };

    if args.len() < 4 {
        print!("No line numbers given.\n");
        std::process::exit(1);
    }

    let line_numbers: &Vec<i32> = &args[3..]
        .iter()
        .map(|x| x.parse::<i32>().unwrap_or(-1))
        .collect();

    let mut out_string: String = "".to_string();
    let mut i: i32 = 0;
    for line in file_content.lines() {
        i += 1;

        if line_numbers.contains(&i) {
            if line.contains("true") {
                out_string.push_str(&line.replace("true", "false"));
            } else if line.contains("false") {
                out_string.push_str(&line.replace("false", "true"));
            } else if line.contains("True") {
                out_string.push_str(&line.replace("True", "False"));
            } else if line.contains("False") {
                out_string.push_str(&line.replace("False", "True"));
            }
        } else {
            out_string.push_str(&line);
        }
        out_string.push_str("\n");
    }
    write(filepath, &out_string).expect("Unable to write file");
}