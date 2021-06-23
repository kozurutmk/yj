extern crate getopts;
use getopts::Options;
use serde_json;
use serde_yaml;
use std::{env, io, process};

const CMD: &str = "yj";
const VERSION: &str = "0.1.0";

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut opts = Options::new();
    opts.optflag("h", "help", "show usage");
    opts.optflag("", "version", "show version");
    let matches = opts.parse(&args[1..])
        .unwrap_or_else(|f| panic!("{}", f.to_string()));
    if matches.opt_present("help") {
        eprintln!("Usage: {} < input.yaml > output.json", CMD);
        process::exit(0);
    }
    if matches.opt_present("version") {
        eprintln!("{} {}", CMD, VERSION);
        process::exit(0);
    }

    let input = Box::new(io::stdin());
    let output = Box::new(io::stdout());

    let data: serde_yaml::Value = serde_yaml::from_reader(input).expect("Failed to read yaml");
    serde_json::to_writer(output, &data).expect("Failed to write json");
}
