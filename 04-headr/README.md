# headr
```shell
$ headr --help
headr 0.1.0
Saurab Tharu <saurab.tharu2@gmail.com>
Rust implementation of head

USAGE:
    headr [OPTIONS] [FILE]...

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c, --bytes <BYTES>    Number of bytes
    -n, --lines <LINES>    Number of lines [default: 10]

ARGS:
    <FILE>...    Input file(s) [default: -]
```




clap::{App, Arg}
std::err::Error

### CLI help

```shell
    short  long  value_name   help
  -----------------------------------------------------
     ||     ||     ||          ||
    -c, --bytes <BYTES>    Number of bytes
    -n, --lines <LINES>    Number of lines [default: 10]
```



### outline of the CLI program

```rust
    let matches = App::new("headr")
            .version()
            .about()
            .author()
            .arg(
                 Arg::with_name()
                    .short()
                    .long()
                    .value_name()
                    .help()
                    .about()
                    // optional files
                    .default_value()
                    .takes_value()
                    .multiple()
                    .conflicts_with()

            )
            .get_matches();
```



### retrieving the value from Command Line Arguments

```rust 
    let lines = matches
            .value_of("lines")      // if the argument is just a string
                                    // `lines` is the value which is given to 
                                    // Arg::with_name(&str) method 
            .map(function_name)     // if you want to parse the value obtained from the `lines`
                                    // map() return Option<Result<usize,Box<>>>
			.transpose()        // convert `Option<Result<usize,Box<>>>` to ` Result<Option<usize,Box<>>>`
                                    // here exchange of `Option` and `Result` occurs
			.map_err(|e| format!("illegal line count -- {}",e))?;



	let files = matches
			.value_of_lossy("files") 	//  gets the invalid UTF-8 code points
			.unwrap();
```



### Guard in matches

```rust
fn some(val: &str) -> MyResult<usize> {
    match val.parse() {
        // here function will return Ok(n) if and only if the parsing is success and `n` is greater than 0 
        // this is guard
        Ok(n) if n>0 => Ok(n), 
        _ => Err(From::from(val)),
    }
}
```



### Reading files

- In Rust, a `String` must be a valid UTF-8-encoded string
- `String::from_utf8` function will return an `Ok` only if the string is valid
- `String::from_utf8_lossy` will convert invalid UTF-8 sequences to the unknown or replacement character

**UTF-8**

```rust
if let Some(num_bytes) = config.bytes {
    let mut handle = file.take(num_bytes as u64); 			// here take() method takes argument as u64
    let mut buffer = vec![0; num_bytes];
    let bytes_read = handle.read(&mut buffer)?; 
    print!(                   
        "{}",
        String::from_utf8_lossy(&buffer[..bytes_read]) 		// converts the valid as well as invalid UTF-8 sequences to String
    ); 
```





**File Operation methods**
