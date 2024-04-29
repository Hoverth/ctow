use std::env;
use std::io;
use std::io::Write;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut cond = true;

    if args.len() > 1 {
        // println!("\x1b[1mHere's your command!\x1b[0m");
        println!("{}", convert(&args[1..]));
    } else {
        // command-line mode
        while cond {
            print!("\x1b[1m> \x1b[0m");
            match io::stdout().flush(){
                Ok(()) => {
                    let mut input = String::new();

                    match io::stdin().read_line(&mut input) {
                        Ok(_n) => {
                            let command = input.split(" ").collect::<Vec<_>>()[0].trim_end();
                            println!("{}", command);
                            match command {
                                "help" => println!("help: prints this message\ncurl [...]: translates a curl command to a wget command\nexit: closes the program"),
                                "curl" => {
                                    let wget = convert(&vec![input.trim_end().to_string()]);
                                    println!("\x1b[1mHere's your command!\x1b[0m\n{}", wget);
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
}

fn convert(curl: &[String]) -> String{
    let curl_args = curl.join(" "); // this makes it all one long string
    let mut args: Vec<String> = vec![];
    let mut url: Vec<String> = vec!["<url>".to_string()];
    //dbg!(&curl_args);
    for arg in curl_args.split(" ") {
        //println!("{:}", arg);
        if arg == "curl" {
            continue;
        } else if arg.starts_with("http") {
            url = vec!(("'".to_owned() + arg + "'").to_string());
        } else if arg.starts_with("-") {
            args.append(&mut vec!(arg.to_string())); 
        } else {
            let len = args.len();
            //println!("{:}", len);
            if len > 0 {
                args[len - 1] += " ";
                args[len - 1] += arg;
            } else {
                panic!("Not a valid argument! {}", arg);
            }
        }
    }
    
    args.append(&mut url);

    let mut wget_args: Vec<String> = Vec::with_capacity(args.len());
    for (i, arg) in args.iter().enumerate() {
        //println!("{}, {}",i ,arg);
        wget_args.insert(i, convert_arg(arg));
    }

    //dbg!("wget".to_owned() + &wget_args.join(" "));
    
    "wget ".to_owned() + &wget_args.join(" ")
}

fn convert_arg(arg: &String) -> String {
    //println!("{}", arg);
    if arg.starts_with("<url>"){
        String::from("<url>")
    } else if arg.starts_with("'http") {
        arg.to_owned()
    } else {
        match arg.split(" ").collect::<Vec<&str>>()[0] {
            "-H" => arg.replace("-H ", "--header '")
                       .replace("\\","\\\\") + "'",
            "--compressed" => arg.replace("--compressed", "--compression=auto"),
            "--connect-timeout" => arg.replace("--connect-timeout ", "--timeout="),
            "--retry" => arg.replace("--retry ", "--tries="),
            _ => {
                println!("No valid substitution for argument: {}!", arg); String::from("")
            }
        }
    }
}
