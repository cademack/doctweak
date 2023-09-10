
#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::super::operations;
    use std::fs::{write, read_to_string};

    static FAKE_TARGET_FILE: &str = "tests/fakeconfig.yml";

    fn read_target_to_tokens() -> Vec<Vec<String>> {
        let target_content = read_to_string(&FAKE_TARGET_FILE).expect("Target file can't be read.");
        let lines_of_tokens: Vec<Vec<String>>= target_content.lines().map(|x| x.split_whitespace().map(str::to_string).collect()).collect();

        return lines_of_tokens;
    }

    fn setup() {
        // reset fakeconfig.yml
        let out_string =
        "debug: True\n\
        cache: True\n\
        path: path/to/thing\n\
        output: False\n\
        type: false\n\
        ".to_string();
        write(&FAKE_TARGET_FILE, &out_string).expect("Unable to write file");
    }

    #[test]
    fn test_toggle_comment() {
        setup();
        let mut args = vec!["mash:", "TOGGLE-COMMENT", &FAKE_TARGET_FILE, "#", "2", "3", "4"].iter().map(|x| x.to_string()).collect();

        operations::toggle_comment(&args);
        let lines_of_tokens = read_target_to_tokens();

        assert_eq!(lines_of_tokens[1][0], "#");
        assert_eq!(lines_of_tokens[2][0], "#");
        assert_eq!(lines_of_tokens[3][0], "#");


        operations::toggle_comment(&args);
        let lines_of_tokens = read_target_to_tokens();

        assert_eq!(lines_of_tokens[1][0], "cache:");
        assert_eq!(lines_of_tokens[2][0], "path:");
        assert_eq!(lines_of_tokens[3][0], "output:");

        args[3] = "//".to_string();
        operations::toggle_comment(&args);
        let lines_of_tokens = read_target_to_tokens();

        assert_eq!(lines_of_tokens[1][0], "//");
        assert_eq!(lines_of_tokens[2][0], "//");
        assert_eq!(lines_of_tokens[3][0], "//");
    }

    #[test]
    fn test_toggle_bool() {
        setup();
        let args = vec!["mash:", "TOGGLE-BOOL", &FAKE_TARGET_FILE, "1", "5"].iter().map(|x| x.to_string()).collect();

        operations::toggle_bool(&args);
        let lines_of_tokens = read_target_to_tokens();

        assert_eq!(lines_of_tokens[0][1], "False");
        assert_eq!(lines_of_tokens[4][1], "true");


        operations::toggle_bool(&args);
        let lines_of_tokens = read_target_to_tokens();

        assert_eq!(lines_of_tokens[0][1], "True");
        assert_eq!(lines_of_tokens[4][1], "false");
    }
}
