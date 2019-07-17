use std::env;
use std::fmt;
use std::fs;

use json::JSON;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = parse_config(&args);

    let contents = fs::read_to_string(config.target).unwrap();

    // println!("{}", contents);
    println!("{:?}", JSON::from(contents));
    // println!("{:?}", JSON::from("[1]"));
    // println!("{:?}", JSON::from("[{\"aa\":[]}]"));

    // println!("{:?}", JSON::from("{\"aaa\":\"big dick\"}"));
    // println!("{:?}", JSON::from("[\"aaa\",\"big dick\"]"));
    // println!("{:?}", JSON::from("[[\"aaa\"],\"aaa\",\"\"]"));
    // println!("{:?}", JSON::from("[[\"aaa\"],\"aaa\",\"\", [\"aa\"]]"));
    // println!("{:?}", JSON::from("[[\"aaa\"],\"aaa\",\"\", [\"aa\", []], []]"));
    // println!("{:?}", JSON::from("[[[[]],[\"aaa\"]], [[[[[{\"aaa\": [[[]]]}]]]]],\"aaa\",\"test\", [\"aa\", []], []]"));
}

fn parse_config(args: &[String]) -> Config {
    let target = if let Some(arg) = args.get(1){
        arg
    } else {
        "-"
    }.to_string();

    let flags = if let Some(arg) = args.get(2){
        arg
    } else {
        ""
    }.to_string();

    Config { target, flags }
}

#[derive(Debug)]
struct Config {
    target: String,
    flags: String,
}

impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.flags)
    }
}
