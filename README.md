# DocTweak

DocTweak is a command line tool to create quick custom commands to manipulate
config files, specifcally .yml files. DocTweak is developed in Rust and serves as a hobby project to learn a new language.

Currently supported operations are are:
TOGGLE-COMMENT: comments or uncomments a line appopriately
TOGGLE-BOOL: finds and replaces boolean values in a line with the opposite value

Custom commands are specified in a commands.config file. Each line is it's own command with the following format:
{command_name}: {OPERATION_LABEL} ({file_path}) arg1 arg2 ...


## TODOs:
- Determine the best way to place config files that users need to design for their custom commands
    - XDG directories, home directories, Windows vs Unix systems... crate: https://docs.rs/xdg/latest/xdg/
- Add more operations
    - SWAP [string1] [string2]
    - tbd
- Figure out how to fully compile/not run through cargo
- Figure out how to distribute