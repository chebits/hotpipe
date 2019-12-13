extern crate clap;

use clap::{Arg, App};
//use std::env;
use std::fs;
use std::io::{self, Read, Write};

use Binding;

fn main() {
    let matches = App::new("HotPipe")
        .version("0.1")
        .author("Chedim <chedim@chedim.com>")
        .about("Manages keyboard shortcuts by reading key events from its pipeline")
        .arg(
            Arg::with_name("input")
            .short("i")
            .long("input")
            .takes_value(true)
            .help("A file to use as input source (use - for stdin)")
        )
        .arg(
            Arg::with_name("config file")
            .short("c")
            .long("config")
            .takes_value(true)
            .help("A config file with hotpipe bindings")
        )
        .arg(
            Arg::with_name("output")
            .short("o")
            .long("output")
            .takes_value(true)
            .help("A file to use as output source (use - for stdout)")
        ).get_matches();

    let input = matches.value_of("input");

    let reader : Box<Read> = match input {
        Some(filename) => Box::new(fs::File::open(filename).unwrap()),
        None => Box::new(io::stdin())
    };

    let output = matches.value_of("output");
    let writer : Box<Write> = match output {
        Some(filename) => Box::new(fs::File::create(filename).unwrap()),
        None => Box::new(io::stdout())
    };


    Ok(())
}


fn read_config(source: Box<Read>) -> Binding {
    let result = Binding::new(0, "")

    let mut key = None;
    for line in source.lines() {
        if let Ok(value) = line {
            if key.is_none() {
                key = Some(value);
            } else {
                
            }
        }
    }
}
