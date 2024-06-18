//!
//! ctow
//! Converts cURL command-line arguments to wget
//!

use std::error;
use std::fmt;

/// Define some constants for pretty printing using ANSI colour codes.
pub const BOLD: &str = "\x1b[1m";
pub const RED: &str = "\x1b[31;1m";
pub const RESET: &str = "\x1b[0m";
pub const GREY: &str = "\x1b[90;3m";

/// Public error types
#[derive(Debug, PartialEq)]
pub enum Errors {
    ArgConversion(String),
    InvalidArgument(String),
    UnrecognisedCommand(String),
}

impl fmt::Display for Errors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Errors::ArgConversion(err) => {
                write!(f, "Conversion: {}", err)
            }
            Errors::InvalidArgument(err) => {
                write!(f, "Invalid Argument: {}", err)
            }
            Errors::UnrecognisedCommand(err) => {
                write!(f, "Unrecognized command: {}", err)
            }
        }
    }
}

impl error::Error for Errors {}

/// converts a curl command (with or without starting with `curl`) to a wget command
///
/// ## Example:
///
/// ```
/// use ctow::convert;
///
/// let input = "curl -H 'User-Agent: Mozilla...'"; 
/// let wget = convert(&[input.to_string()]);
/// ```
pub fn convert(curl: &[String]) -> Result<String, Errors> {
    let curl_args = curl.join(" "); // this makes the input all one long string
    let mut args: Vec<String> = vec![];
    let mut url: Vec<String> = vec!["<url>".to_string()];

    for arg in curl_args.split(' ') {
        if arg == "curl" {
            continue; // discard a "curl" - bugfix needed, only remove curl at start of command.
        } else if arg.starts_with("http") {
            // if there is a " http", assume that it's the url (grabs the last one in the command)
            url = vec![("'".to_owned() + arg + "'").to_string()];
        } else if arg.starts_with('-') {
            // if the arg starts with a dash, assume it's a new argument
            args.append(&mut vec![arg.to_string()]);
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

/// Converts a curl argument to a wget argument
///
/// ## Example
///
/// ```
/// use ctow::convert_arg;
///
/// let curl_argument = "-H 'User-Agent: Mozilla...'";
/// let wget_argument = convert_arg(curl_argument);
/// ```
pub fn convert_arg(arg: &str) -> Result<String, Errors> {
    // if it's the url, don't touch it
    if arg.starts_with("<url>") {
        Ok(String::from("<url>"))
    } else if arg.starts_with("'http") {
        Ok(arg.to_owned())
    } else {
        // else, replace the curl with the wget
        match arg.split(' ').collect::<Vec<&str>>()[0] {
            "-b" => Ok(arg.replace("-b ", "--load-cookies=")),
            "-c" => Ok(arg.replace("-c ", "--save-cookies=")),
            "-d" => Ok(arg.replace("-d ", "--post-data=")),
            "-e" => Ok(arg.replace("-e ", "--header=\"Referer: ") + "\""),
            "-g" => Ok(arg.replace("-g", "--no-glob")),
            "-k" => Ok(arg.replace("-k", "--no-check-certificate")),
            "-m" => Ok(arg.replace("-m ", "--timeout=")),
            "-o" => Ok(arg.replace("-o ", "--output-document=")),
            "-r" => Ok(arg.replace("-r ", "--header=\"Range: bytes=") + "\""),
            "-s" => Ok(arg.replace("-s", "--quiet")),
            "-u" => Ok(arg.replace("-u ", "--user=")),
            "-z" => Ok(arg.replace("-z ", "--header=\"If-Modified-Since: ") + "\""),

            "-A" => Ok(arg.replace("-A ", "--header=\"User-Agent: ") + "\""),
            "-C" => Ok(arg.replace("-C ", "--start-pos=")),
            "-E" => Ok(arg.replace("-E ", "--certificate=")),
            "-H" => Ok(arg.replace("-H ", "--header '").replace('\\', "\\\\") + "'"),
            "-I" => Ok(arg.replace("-I", "--method=HEAD")),
            "-T" => Ok(arg.replace("-T ", "--method=PUT --body-file=")),
            "-X" => Ok(arg.replace("-X ", "--method=")),

            "--compressed" => Ok(arg.replace("--compressed", "--compression=auto")),
            "--connect-timeout" => Ok(arg.replace("--connect-timeout ", "--timeout=")),
            "--retry" => Ok(arg.replace("--retry ", "--tries=")),
            _ => Err(Errors::ArgConversion(format!(
                "{RED}No valid substitution for argument: {arg}!{RESET}",
            ))),
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
        assert_eq!(
            convert(&[test_curl1]),
            Ok("wget 'http://example.com'".into())
        );
    }

    #[test]
    fn test_convert_args() {
        // in this current format, ctow does not support -<option><var> (e.g. -bLOAD-COOKIE) there
        // has to be a space in between option and var
        let test_args = vec![
            //"-bLOAD-COOKIE",
            "-b LOAD-COOKIE",
            //"-cSAVE-COOKIE", 
            "-c SAVE-COOKIE",
            //"-dPOST-DATA",
            "-d POST-DATA",
            //"-eREFERER",
            "-e REFERER",
            "-g",
            "-k",
            //"-m999",
            "-m 9999",
            //"-oOUTPUT",
            "-o OUTPUT",
            //"-r1-2",
            "-r 2-3",
            "-s",
            //"-uUSER",
            "-u USER",
            //"-zMODIFIED",
            "-z MODIFIED",

            //"-AAGENT",
            "-A AGENT",
            //"-C1",
            "-C 2",
            //"-ECERT",
            "-E CERT",
            "-H User-Agent: Example",
            "-I",
            //"-TBODY",
            "-T BODY",
            //"-XMETHOD",
            "-X METHOD",

            "--compressed",
            "--connect-timeout 5",
            "--retry 3",
        ];
        let result_args = vec![
            //"--load-cookies=LOAD-COOKIE",
            "--load-cookies=LOAD-COOKIE",
            //"--save-cookies=SAVE-COOKIE",
            "--save-cookies=SAVE-COOKIE",
            //"--post-data=POST-DATA",
            "--post-data=POST-DATA",
            //"--header=\"Referer: REFERER\"",
            "--header=\"Referer: REFERER\"",
            "--no-glob",
            "--no-check-certificate",
            //"--timeout=999",
            "--timeout=9999",
            //"--output-document=OUTPUT",
            "--output-document=OUTPUT",
            //"--header=\"Range: bytes=1-2\"",
            "--header=\"Range: bytes=2-3\"",
            "--quiet",
            //"--user=USER",
            "--user=USER",
            //"--header=\"If-Modified-Since: MODIFIED\"",
            "--header=\"If-Modified-Since: MODIFIED\"",

            //"--header=\"User-Agent: AGENT\"",
            "--header=\"User-Agent: AGENT\"",
            //"--start-pos=1",
            "--start-pos=2",
            //"--certificate=CERT",
            "--certificate=CERT",
            "--header 'User-Agent: Example'",
            "--method=HEAD",
            //"--method=PUT --body-file=BODY",
            "--method=PUT --body-file=BODY",
            "--method=METHOD",

            "--compression=auto",
            "--timeout=5",
            "--tries=3",
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
