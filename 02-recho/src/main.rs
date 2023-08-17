use clap::{App, Arg};

fn main() {
    let matches = App::new("recho")
        .version("0.1.0")
        .author("Saurab tharu <saurab.tharu2@gmail.com>")
        .about("Echo implementation in Rust")
        .arg(
            Arg::with_name("text")
                .value_name("TEXT")
                .help("Input text")
                .required(true)
                .min_values(1),
        )
        .arg(
            Arg::with_name("omit_newline")
                .short("n")
                .help("Do not print the new line")
                .takes_value(false),
        )
        .get_matches();

    let text = matches.values_of_lossy("text").unwrap(); // for value of argument with name "text"

    let omit_newline = matches.is_present("omit_newline"); // for value of argument  with name "omit_newline"
                                                           /*
                                                           let mut ending = "\n";
                                                           if omit_newline {
                                                               ending = "";
                                                           }
                                                           */
    let ending = if omit_newline { "" } else { "\n" };
    print!("{}{}", text.join(" "), ending);

    // println!("{:#?}", matches);
}
