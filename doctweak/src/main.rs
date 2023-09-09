use std::env;

pub mod operations;
pub mod config;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print!("No command provided.\nCorrect usage is `doctweak <command>`\n");
        std::process::exit(1);
    }
    
    let config_content = config::get_config_content();    

    // Iterate through each line of the commands.config to find the appropriate for the given argument
    for line in config_content.lines() {
        let tokens: Vec<String> = line.split_whitespace().map(str::to_string).collect();
        
        // Acquire first token and peel off ending colon to check the given command
        let first_token = match tokens.get(0) {
            Some(str) => str,
            None => panic!("Empty line in config file")
        };
        let command = &first_token[0..first_token.len()-1];

        // Don't need to continue here if this is the wrong command-line
        if !command.eq(args.get(1).unwrap()) {
            continue;
        }

        // Operation has no extra characters, so can pull it directly
        let operation = match tokens.get(1) {
            Some(str) => str,
            None => panic!("No operation on one line of config file")
        };

        match operation.as_str() {
            "TOGGLE-COMMENT" => {
                operations::toggle_comment_line(&tokens);
                return;
            },
            "TOGGLE-BOOL" => {
                operations::toggle_bool_line(&tokens);
                return;
            },
            &_ => return,
        }
    }
    print!("Command not found.\n");
    std::process::exit(1);
}
