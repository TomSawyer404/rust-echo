use clap::{App, Arg};

fn main() {
    let matches = App::new("rust-echo")
        .author("tomsawyer404@outlook.com")
        .version("0.1.0")
        .about("`echo` utility written in Rust")
        .arg(
            Arg::with_name("text")
                .value_name("TEXT")
                .help("Please input some words") // `help` is a must
                .required(true)
                .min_values(1),
        )
        .arg(
            Arg::with_name("omit_newline")
                .help("do not output the trailing newline")
                .takes_value(false)
                .short("n"),
        )
        .get_matches();

    let text = matches.values_of_lossy("text").unwrap();
    let omit_newline = matches.is_present("omit_newline");
    print!("{}{}", text.join(" "), if omit_newline { "" } else { "\n" });
    //println!("{:#?}", matches);
}
