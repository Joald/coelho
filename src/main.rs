mod image;

use crate::image::create_fake_quote;
use chrono::prelude::*;
use clap::{App, Arg};
use std::io::{self, Read};
use std::path::PathBuf;

fn generate_file_name() -> String {
    let mut res = String::new();
    res.push_str("paulo_coelho_");
    res.push_str(&format!("{}", Local::now().format("%Y_%m_%d_%H_%M_%S_%f")));
    res.push_str(".png"); // TODO: add support for different extensions
    res
}

fn main() -> io::Result<()> {
    let matches = App::new("coelho - Paulo Coelho Fake Quote Generator")
        .version("0.1")
        .author("Jacek Olczyk <jacek.olczyk98@gmail.com>")
        .arg(
            Arg::with_name("output")
                .short("o")
                .long("output")
                .takes_value(true)
                .value_name("TEXT")
                .help("Output file name"),
        )
        .arg(
            Arg::with_name("multiline")
                .short("m")
                .conflicts_with("quote")
                .help("Allow for multiple lines in the quote. Line breaks may be removed."),
        )
        .arg(
            Arg::with_name("quote")
                .short("q")
                .long("quote")
                .takes_value(true)
                .value_name("TEXT")
                .help("Quote to misattribute"),
        )
        .get_matches();

    let quote = matches
        .value_of("quote")
        .map(|val| val.to_owned())
        .or_else(|| {
            let multiline = matches.is_present("multiline");
            if atty::is(atty::Stream::Stdin) {
                if multiline {
                    println!("Enter quote until EOF");
                } else {
                    println!("Enter quote");
                }
            }
            let mut buffer = String::new();
            if multiline {
                io::stdin().read_to_string(&mut buffer).ok()?;
            } else {
                io::stdin().read_line(&mut buffer).ok()?;
            }
            Some(buffer)
        })
        .unwrap_or_default();

    let mut output_file = matches
        .value_of("output")
        .map(PathBuf::from)
        .unwrap_or(std::env::current_dir()?);
    if output_file.is_dir() {
        output_file.push(generate_file_name());
    }
    println!("{}", quote);
    println!("{}", output_file.to_str().unwrap());
    create_fake_quote(quote, output_file)
}
