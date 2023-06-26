Doctweak is a command line tool to create quick custom commands to manipulate
config files, specifcally .yml files. Doctweak is developed in Rust and serves as a hobby project to learn a new language.

Currently supported operations are are:
TOGGLE-COMMENT: comments or uncomments a line appopriately
TOGGLE-BOOL: finds and replaces boolean values in a line with the opposite value

Custom commands are specified in a commands.config file. Each line is it's own command with the following format:
{command_name}: {OPERATION_LABEL} ({file_path}) arg1 arg2 ...