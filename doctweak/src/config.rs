use std::fs::read_to_string;
use std::string::String;

use directories::ProjectDirs;

pub fn get_config_content() -> String {
    let proj_dirs = ProjectDirs::from("", "", "doctweak").unwrap();
    let config_dir_path = proj_dirs.config_dir(); 

    let mut config_dir_path_buf = config_dir_path.to_path_buf();
    config_dir_path_buf.push("doctweak.config");

    if config_dir_path_buf.exists() {
        let config_file_contents = read_to_string(config_dir_path_buf).unwrap();
        return config_file_contents;
    } else {
        panic!("No config file found! Please place a `doctweak.config` at\n
                {}", config_dir_path_buf.display());
    }
}