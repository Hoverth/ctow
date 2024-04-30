# CTOW

cURL to Wget

A simple rust binary utility to convert curl commands to wget commands.

Available via `cargo install ctow` ([link to crates.io](https://crates.io/crates/ctow)).

## Docs

ctow can be used in two modes: command with arguments and as a command line interface

### Command With Arguments

You can simply pass `ctow [curl command]`, and ctow will print the converted command to stdout, with no formatting or anything when successful, so the output can be piped and manipulated into a command like `wget $(ctow [...])`, etc

Do note that when passing in a curl command, you do not explicitly need to include the leading `curl`, as it is discarded anyway.

### Command-Line Interface

ctow comes with a command-line interface, which **does** have formatted output, and is not designed for use with scripts, but for a more user-friendly, interactive experience.  

There are three commands:

- `help` - prints a help message
- `curl` - will convert a curl command into a wget command
- `exit` - exits the ctow CLI

## Contributing

All code is GPLv3+ licensed, and by contributing you agree to license the contributed code under this license.

Contributions are welcome for:

- missing curl / wget argument mappings
- test cases
- anything else, please open an issue beforehand

## License

Copyright © 2024 Thomas Dickson and other contributors

This code is license under the GNU GPL v3+. See LICENSE.md for more details
