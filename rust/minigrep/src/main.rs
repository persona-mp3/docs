#![allow(dead_code)]
use std::env;
use std::error::Error;
use std::fs;
use std::process;

use minigrep::search;

const MIN_ARGS: u8 = 3;

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        let query = args[1].clone();
        let file_path = args[2].clone();
        Config { query, file_path }
    }

    // Very Interesting...
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments, provide 'query', 'source_file' ");
        }

        // let query = args[1].clone();
        // let file_path = args[2].clone();
        // Ok(Config { query, file_path })
        return Ok(Config::new(args));
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    // if args.len() < MIN_ARGS as usize {
    //     eprintln!("please provide 'search_term' 'file_path'");
    //     return;
    // }

    // Is funny how you could do this in different ways
    //
    // let config = parse_config(&args);
    // let config = Config::new(&args);
    // Something like this in go:
    // type Config {
    //  query     string
    //  file_path string
    // }
    //
    // type Builder interface {
    //  Build(args []string)
    // }
    //
    // func (b Builder) Build(args []string) (*Config, err) {}
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    // I understand the move_semantics, but I don't really see the point
    // of dropping the whole value out of scope if a function doesn't consume
    // it or take a mutable reference it. Would make sense in 'concurrent' or lower level
    // programming where, you actually manage memory? But yea
    // println!("Query: {} | Source: {}", config.query, config.file_path);
    // let file_content = match fs::read_to_string(&config.file_path) {
    //     Ok(f) => f,
    //     Err(err) => {
    //         eprintln!("Could not open {}. Reason: {}", &config.file_path, err);
    //         return;
    //     }
    // };
    //
    let file_content = run(&config).unwrap_or_else(|err| {
        eprintln!("Error Running: {err}");
        process::exit(1);
    });
    println!("{}", file_content);
}

fn run(cfg: &Config) -> Result<String, Box<dyn Error>> {
    let file_content = fs::read_to_string(&cfg.file_path)?;
    for line in search(&cfg.query, &cfg.file_path) {
        println!("{line}");
    }

    Ok(file_content)
    // Ok((file_content))
}

fn parse_config(args: &Vec<String>) -> Config {
    let query = args[1].clone();
    let file_path = args[2].clone();
    Config { query, file_path }
}
