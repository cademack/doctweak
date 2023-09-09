# DocTweak

Tired of opening Vim for the 8th time in 5 minutes just to flip a boolean
in your config file? How about just to comment out a line?

If so, you have come to the right place :).

DocTweak is a command line tool to create quick custom commands to manipulate
config files, specifically with .yml files in mind. DocTweak is developed in Rust
and serves as a hobby project to learn a new language.

## Configuration
Custom commands are specified in a commands.config file. Each line is it's own
command with the following format:

{command_name}: {OPERATION_LABEL} arg1 arg2 ...

The configuration file is searched for in the XDG Config directory as `doctweak.yml`.
Determining the XDG config directory is accomplished with the
[directories](https://docs.rs/directories/latest/directories/) crate.

## Operations
- TOGGLE-COMMENT: comments or uncomments a line appopriately with a given comment prefix.
a space is added to the prefix before it is detected and added. thats how I like it
    - TOGGLE-COMMENT <filepath> <comment prefix> <line number 1> <line number 2> ...
        + comment prefix = "#" "\\" etc.
- TOGGLE-BOOL: finds and replaces boolean values in a line with the opposite
value, ex: True <=> False, true <=> false
    - TOGGLE-BOOL <filepath> <line number 1> <line number 2> ...

## TODOs:
- Add more operations
    - SWAP [string1] [string2]
    - tbd
- Figure out how to fully compile/not run through cargo
- Figure out how to distribute

## Changenotes

- 9/8/2023: changed operations to take the vector of args, should enable more diversity
in operations, better control logic, toggle_bool handles capitalized and uncapitalized.
configuration file is first searched for in current working directory, then xdg config.
rustified the logic, lots of matches