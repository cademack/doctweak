use std::env;

pub mod operations;
pub mod config;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print!("No command provided.\nCorrect usage is `doctweak <command>`\n");
        std::process::exit(1);
    }
    
    let config_content = match config::get_config_content() {
        Some(content) => content,
        None => {
            print!("Configuration file cannot be found or read.\n");
            std::process::exit(1);
        }
    };

    // Iterate through each line of the commands.config to find the appropriate for the given argument
    for line in config_content.lines() {
        let tokens: Vec<String> = line.split_whitespace().map(str::to_string).collect();
        
        if tokens.len() < 3 {
            // not enough tokens on this line to be a valid command
            print!("Invalid command encountered. The minimum command definition is:\n
                    <command>: <operation> <filepath/arg1>");
            continue;
        }

        let first_token = match tokens.get(0) {
            Some(str) => str,
            None => panic!("Empty line in config file")
        };
        let command = &first_token[0..first_token.len()-1];

        // Don't need to continue here if this is the wrong command-line
        if !command.eq(args.get(1).unwrap()) {
            continue;
        }

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
