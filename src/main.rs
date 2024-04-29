use std::env;
use std::io::{self, Write};

#[derive(Debug, PartialEq)]
enum Errors {
    ArgConversion(String),
    InvalidArgument(String),
    UnrecognisedCommand(String),
}

impl std::fmt::Display for Errors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Errors::ArgConversion(err) => {
                write!(f, "Conversion: {}", err)
            },
            Errors::InvalidArgument(err) => {
                write!(f, "Invalid Argument: {}", err)
            },
            Errors::UnrecognisedCommand(err) => {
                write!(f, "Unrecognized command: {}", err)
            }
        }
    }
}

impl std::error::Error for Errors {}

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
            match io::stdout().flush(){
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

// converts a curl command (with or without starting with `curl`) to a wget command
fn convert(curl: &[String]) -> Result<String, Errors> {
    let curl_args = curl.join(" "); // this makes the input all one long string
    let mut args: Vec<String> = vec![];
    let mut url: Vec<String> = vec!["<url>".to_string()];

    for arg in curl_args.split(' ') {
        if arg == "curl" {
            continue; // discard a "curl" - bugfix needed, only remove curl at start of command.
        } else if arg.starts_with("http") {
            // if there is a " http", assume that it's the url (grabs the last one in the command)
            url = vec!(("'".to_owned() + arg + "'").to_string());
        } else if arg.starts_with('-') {
            // if the arg starts with a dash, assume it's a new argument
            args.append(&mut vec!(arg.to_string())); 
        } else {
            // else, append the rest of the arg to the previous arg, this helps with arguments
            // with spaces in them
            let len = args.len();
            if len > 0 {
                args[len - 1] += " ";
                args[len - 1] += arg;
            } else {
                return Err(Errors::InvalidArgument(arg.to_string()));
            }
        }
    }
    
    args.append(&mut url); // append the url last

    // converts the arg from curl to wget
    let mut wget_args: Vec<String> = Vec::with_capacity(args.len());
    for (i, arg) in args.iter().enumerate() {
        wget_args.insert(i, convert_arg(arg)?);
    }

    Ok("wget ".to_owned() + &wget_args.join(" "))
}

// converts a curl argument to a wget argument
fn convert_arg(arg: &str) -> Result<String, Errors> {
    // if it's the url, don't touch it
    if arg.starts_with("<url>"){
        Ok(String::from("<url>"))
    } else if arg.starts_with("'http") {
        Ok(arg.to_owned()) 
    } else {
        // else, replace the curl with the wget
        match arg.split(' ').collect::<Vec<&str>>()[0] {
            "-H" => Ok(arg.replace("-H ", "--header '")
                       .replace('\\',"\\\\") + "'"),
            "--compressed" => Ok(arg.replace("--compressed", "--compression=auto")),
            "--connect-timeout" => Ok(arg.replace("--connect-timeout ", "--timeout=")),
            "--retry" => Ok(arg.replace("--retry ", "--tries=")),
            _ => Err(Errors::ArgConversion(format!("\x1b[1;31mNo valid substitution for argument: {}!\x1b[0m", arg)))
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_convert() {
        // tests the conversion of a whole command
        let test_curl1 = "curl http://example.com".to_string();
        assert_eq!(convert(&[test_curl1]), Ok("wget 'http://example.com'".into()));
    }

    #[test]
    fn test_convert_args() {
        let test_args = vec![
            "--compressed"
        ];
        let result_args = vec![
            "--compression=auto"
        ];
        
        for (i, test_arg) in test_args.iter().enumerate() {
            assert_eq!(convert_arg(test_arg), Ok(result_args[i].to_string()));
        }
    }

    #[test]
    fn test_convert_url() {
        let test_str1 = "'http";

        assert_eq!(convert_arg(test_str1), Ok("'http".into()));
    }
}
