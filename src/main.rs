use std::env;
use std::io::{self, Write};

#[derive(Debug, PartialEq)]
enum Errors {
    ArgConversion(String),
    InvalidArgument(String),
    UnrecognisedCommand,
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
            Errors::UnrecognisedCommand => {
                write!(f, "Unrecognized command")
            }
        }
    }
}

impl std::error::Error for Errors {}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    let mut cond = true;

    if args.len() > 1 {
        // println!("\x1b[1mHere's your command!\x1b[0m");
        println!("{}", convert(&args[1..])?);
    } else {
        // command-line mode
        while cond {
            print!("\x1b[1m> \x1b[0m");
            match io::stdout().flush(){
                Ok(()) => {
                    let mut input = String::new();

                    match io::stdin().read_line(&mut input) {
                        Ok(_n) => {
                            let command = input.split(' ').collect::<Vec<_>>()[0].trim_end();
                            println!("{}", command);
                            match command {
                                "help" => println!("help: prints this message\ncurl [...]: translates a curl command to a wget command\nexit: closes the program"),
                                "curl" => {
                                    let wget = convert(&[input.trim_end().to_string()]);
                                    match wget {
                                        Ok(wget) => println!("\x1b[1mHere's your command!\x1b[0m\n{}", wget),
                                        Err(err) => eprintln!("\x1b[1mError encountered: {}", err),
                                    }
                                }
                                "exit" => cond = false,
                                _ => println!("Unrecognised command: {command}"),
                            }
                        }
                        Err(error) => println!("There was an error: {error}"),
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

fn convert(curl: &[String]) -> Result<String, Errors> {
    let curl_args = curl.join(" "); // this makes it all one long string
    let mut args: Vec<String> = vec![];
    let mut url: Vec<String> = vec!["<url>".to_string()];
    //dbg!(&curl_args);
    for arg in curl_args.split(' ') {
        //println!("{:}", arg);
        if arg == "curl" {
            continue;
        } else if arg.starts_with("http") {
            url = vec!(("'".to_owned() + arg + "'").to_string());
        } else if arg.starts_with('-') {
            args.append(&mut vec!(arg.to_string())); 
        } else {
            let len = args.len();
            //println!("{:}", len);
            if len > 0 {
                args[len - 1] += " ";
                args[len - 1] += arg;
            } else {
                return Err(Errors::InvalidArgument(arg.to_string()));
            }
        }
    }
    
    args.append(&mut url);

    let mut wget_args: Vec<String> = Vec::with_capacity(args.len());
    for (i, arg) in args.iter().enumerate() {
        //println!("{}, {}",i ,arg);
        wget_args.insert(i, convert_arg(arg)?);
    }

    //dbg!("wget".to_owned() + &wget_args.join(" "));
    
    Ok("wget ".to_owned() + &wget_args.join(" "))
}

fn convert_arg(arg: &str) -> Result<String, Errors> {
    //println!("{}", arg);
    if arg.starts_with("<url>"){
        Ok(String::from("<url>"))
    } else if arg.starts_with("'http") {
        Ok(arg.to_owned())
    } else {
        match arg.split(' ').collect::<Vec<&str>>()[0] {
            "-H" => Ok(arg.replace("-H ", "--header '")
                       .replace('\\',"\\\\") + "'"),
            "--compressed" => Ok(arg.replace("--compressed", "--compression=auto")),
            "--connect-timeout" => Ok(arg.replace("--connect-timeout ", "--timeout=")),
            "--retry" => Ok(arg.replace("--retry ", "--tries=")),
            _ => Err(Errors::ArgConversion(format!("No valid substitution for argument: {}!", arg)))
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_convert() {
        let test_curl1 = "curl http://google.com".to_string();
        assert_eq!(convert(&[test_curl1]), Ok("wget 'http://google.com'".into()));
    }

    #[test]
    fn test_convert_arg() {
        let test_str1 = "'http";

        assert_eq!(convert_arg(test_str1), Ok("'http".into()));
    }
}