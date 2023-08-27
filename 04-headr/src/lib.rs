use clap::{App, Arg};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,   // for taking multiple files
    lines: usize,         // number of lines to print
    bytes: Option<usize>, // bytes to be optional
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("headr")
        .version("0.1.0")
        .author("Saurab Tharu <saurab.tharu2@gmail.com>")
        .about("Rust implementation of head")
        .arg(
            Arg::with_name("lines")
                .short("n")
                .long("lines")
                .value_name("LINES")
                .help("Number of lines")
                // .takes_value(true),
                .default_value("10"),
        )
        .arg(
            Arg::with_name("bytes")
                // SLVH
                /*
                 * `short`
                 * `long`
                 * `value_name`
                 * `help`
                 * */
                .short("c")
                .long("bytes")
                .value_name("BYTES")
                .help("Number of bytes ")
                .takes_value(true)
                .conflicts_with("lines"),
        )
        .arg(
            Arg::with_name("files")
                .value_name("FILE")
                .help("Input file(s)")
                .multiple(true)
                .default_value("-"),
        )
        .get_matches();

    let lines = matches
        .value_of("lines")
        .map(parse_positive_int)
        .transpose()
        .map_err(|e| format!("illegal line count -- {}", e))?;

    let bytes = matches
        .value_of("bytes")
        .map(parse_positive_int)
        .transpose()
        .map_err(|e| format!("illegal byte count -- {}", e))?;

    Ok(Config {
        files: matches.values_of_lossy("files").unwrap(),
        lines: lines.unwrap(), /* `lines` contain Option<usize> and since Config::lines
                               need `usize` it is unwrapped */

        bytes, // `bytes` contain Option<usize> and since Config::bytes need
               // Option<usize> so it is not wrapped
    })
}

pub fn run(config: Config) -> MyResult<()> {
    let num_files = config.files.len();

    for (file_num, filename) in config.files.iter().enumerate() {
        match open(&filename) {
            Err(err) => eprintln!("{}: {}", filename, err),
            // Ok(_) => println!("Opened {}", filename),
            Ok(mut file) => {
                if num_files > 1 {
                    println!(
                        "{}==> {} <==",
                        if file_num > 0 { "\n" } else { "" },
                        filename
                    );
                }

                if let Some(num_bytes) = config.bytes {
                    let bytes: Result<Vec<_>, _> = file.bytes().take(num_bytes).collect();
                    print!("{}", String::from_utf8_lossy(&bytes?));

                    /*
                    let mut handle = file.take(num_bytes as u64);
                    let mut buffer = vec![0; num_bytes];
                    let bytes_read = handle.read(&mut buffer)?;
                    print!("{}", String::from_utf8_lossy(&buffer[..bytes_read]));
                    */
                } else {
                    let mut line = String::new();
                    for _ in 0..config.lines {
                        let bytes = file.read_line(&mut line)?;
                        if bytes == 0 {
                            break;
                        }
                        print!("{}", line);
                        line.clear();
                    }
                }

                // for line in file.lines().take(config.lines) {
                //     println!("{}", line?);
                // }
                /*
                println!();
                if config.files.len() > 1 {
                    println!("\n==> {} <==", filename);
                }

                for (linenum, line) in file.lines().enumerate() {
                    if linenum == config.lines {
                        break;
                    }
                    let line = line?;
                    println!("{}", line);
                }
                */
            }
        }
    }
    Ok(())
}

fn parse_positive_int(val: &str) -> MyResult<usize> {
    match val.parse() {
        Ok(n) if n > 0 => Ok(n),
        _ => Err(From::from(val)),
    }
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        // if there is no filename in command line default argument will be "-"
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        // for other argument than "-"
        // `?` for error propagation
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

#[test]
fn test_parse_positive_int() {
    // 3 is and OK integer
    let res = parse_positive_int("3");
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), 3);

    // Any string is an error
    let res = parse_positive_int("foo");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "foo".to_string());

    // A zero is an error
    let res = parse_positive_int("0");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "0".to_string());
}
