use std::fs::read_to_string;
use std::string::String;

use directories::ProjectDirs;

pub fn get_config_content() -> Option<String> {

    match std::env::current_dir() {
        Ok(mut cwd_path_buf) => {
            cwd_path_buf.push("doctweak.config");
            if cwd_path_buf.exists() {
                // print!("Using configuration file within current working directory.\n"); only for debugging
                match read_to_string(cwd_path_buf) {
                    Ok(contents) => return Some(contents),
                    _ => (),
                }
            }
        }
        _=> print!("Current working directory cannot be found.\n"),
    };

    match ProjectDirs::from("", "", "doctweak") {
        Some(project_dirs) => {
            let config_dir_path = project_dirs.config_dir(); 

            let mut config_dir_path_buf = config_dir_path.to_path_buf();
            config_dir_path_buf.push("doctweak.config");

            if config_dir_path_buf.exists() {
                match read_to_string(config_dir_path_buf) {
                    Ok(contents) => return Some(contents),
                    Err(_) => (),
                }
            } else {
                print!("No config file found! Please place a `doctweak.config` at\n
                        {}\n", config_dir_path_buf.display());
                return None;
            }
        },
        None => {
            print!("XDG Config directory cannot be found.\n");
            return None;
        }
    };
    return None;
}