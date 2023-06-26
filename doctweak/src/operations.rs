use std::fs::{read_to_string, write};
use std::path::PathBuf;


pub fn toggle_comment_line(filepath: &PathBuf, line_number: i32) {
    let mut out_string: String = "".to_string();
    let mut i: i32 = 0;
    for line in read_to_string(filepath).unwrap().lines() {
        let comment_prefix = "# ";
        i += 1;
        
        if i == line_number {
            if &line[0..2] == comment_prefix {
                out_string.push_str(&line[2..]);
            } else {
                out_string.push_str(&comment_prefix);
                out_string.push_str(&line);
            }
        } else {
            out_string.push_str(&line)
        }
        out_string.push_str("\n");
    }
    write(filepath, &out_string).expect("Unable to write file");
}

pub fn toggle_bool_line(filepath: &PathBuf, line_number: i32) {
    let mut out_string: String = "".to_string();
    let mut i: i32 = 0;
    for line in read_to_string(filepath).unwrap().lines() {
        i += 1;
        
        if i == line_number {
            if line.contains("true") {
                out_string.push_str(&line.replace("true", "false"))
            } else if line.contains("false") {
                out_string.push_str(&line.replace("false", "true"))
            }
        } else {
            out_string.push_str(&line)
        }
        out_string.push_str("\n");
    }
    write(filepath, &out_string).expect("Unable to write file");
}