use std::io::{self, Write};

pub fn run(args: &[String]) {
    print!("{}", args.join(" "));
    io::stdout().flush().unwrap();
}
