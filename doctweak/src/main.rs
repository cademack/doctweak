use clap::Parser;
use std::fs::{read_to_string};
use std::path::PathBuf;

pub mod operations;

#[derive(Parser)]
struct Cli {
    command: String,
}

fn main() {
    let args = Cli::parse();
    let filename = "tests\\commands.config";

    // Iterate through each line of the commands.config to find the appropriate for the given argument
    for line in read_to_string(filename).unwrap().lines() {
        let mut tokens = line.split_whitespace();
        
        // Acquire first token and peel off ending colon to check the given command
        let first_token = match tokens.next() {
            Some(str) => str,
            None => panic!("Empty line in config file")
        };
        let command = &first_token[0..first_token.len()-1];

        // Don't need to continue here if this is the wrong command-line
        if command != args.command {
            continue;
        }

        // Operation has no extra characters, so can pull it directly
        let operation = match tokens.next() {
            Some(str) => str,
            None => panic!("No operation on one line of config file")
        };

        // Filepath is surrounded in parenthesis
        let third_token = match tokens.next() {
            Some(str) => str,
            None => panic!("No operation on one line of config file")
        };
        let filepath_str = &third_token[1..third_token.len()-1];
        let filepath = PathBuf::from(&filepath_str);

        // Fourth token is line-index for the operation
        let fourth_token = match tokens.next() {
            Some(str) => str,
            None => panic!("No operation on one line of config file")
        };
        let line_index = fourth_token.parse::<i32>().unwrap();

        match operation {
            "TOGGLE-COMMENT" => {
                operations::toggle_comment_line(&filepath, line_index);
                return;
            },
            "TOGGLE-BOOL" => {
                operations::toggle_bool_line(&filepath, line_index);
                return;
            },
            &_ => break,
        }
    }
    panic!("Command not found.")
}
