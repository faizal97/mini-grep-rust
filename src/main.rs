use std::env;
use std::process;

use mini_grep_rust::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args.as_slice()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = mini_grep_rust::run(config) {
        eprintln!("Application error: {}", e);

        if e.to_string().contains("regex parse error") {
            eprintln!("Hint: Check your regex pattern syntax");
        }

        process::exit(1);
    }
}
