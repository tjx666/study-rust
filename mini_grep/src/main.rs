use std::{
    env::{self, Args},
    fs,
};

fn main() {
    let config = Config::parse(env::args());
    let contents = fs::read_to_string(config.filename).expect("read file error");
    println!("{}", contents);

    let s = String::from("123");
    let s1 = s;
    println!("{}", s1);
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn parse(mut args: Args) -> Config {
        args.next();
        Config {
            query: args.next().unwrap(),
            filename: args.next().unwrap(),
        }
    }
}
