use std::env;
use std::io::{self, Write};

use ctow::{convert, Errors};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        // argument mode

        // don't print anything here, just in case you want to pipe into wget
        println!("{}", convert(&args[1..])?);
    } else {
        // command-line mode

        let mut cond = true;

        while cond {
            // enter the cli, with a nice ">" prompt
            print!("\x1b[1m> \x1b[0m");
            match io::stdout().flush() {
                Ok(()) => {
                    // if writing stdout worked, get the next line of input
                    let mut input = String::new();

                    match io::stdin().read_line(&mut input) {
                        Ok(_n) => {
                            // get and match the command
                            let command = input.split(' ').collect::<Vec<_>>()[0].trim_end();
                            println!("\x1b[3;90m{}\x1b[0m", command);
                            match command {
                                "help" => println!("help: prints this message\ncurl [...]: translates a curl command to a wget command\nexit: closes the program"),
                                "curl" => {
                                    let wget = convert(&[input.trim_end().to_string()]);
                                    match wget {
                                        Ok(wget) => println!("\x1b[1mHere's your command!\x1b[0m\n{}", wget),
                                        Err(err) => eprintln!("\x1b[1;31mError encountered: {}\x1b[0m", err),
                                    }
                                }
                                "exit" => cond = false,
                                _ => eprintln!("\x1b[1;31mUnrecognised command: {}\x1b[0m", Errors::UnrecognisedCommand(command.to_string())),
                            }
                        }
                        Err(error) => eprintln!("\x1b[1;31mThere was an error: {error}\x1b[0m"),
                    }
                }
                Err(error) => {
                    println!("There was an error: {error}")
                }
            }
        }
    }
    Ok(())
}
