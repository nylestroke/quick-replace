use regex::Regex;
use std::{env, fs};
use text_colorizer::*;

#[derive(Debug)]
struct Arguments {
    target: String,
    replacement: String,
    filename: String,
    output: String,
}

fn print_usage() {
    eprintln!(
        "{} - replaces all occurrences of a string with another one",
        "quick-replace".green()
    );
    eprintln!("How to use: quick-replace <target> <replacement> <input_file> <output_file>");
}

fn parse_args() -> Arguments {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() != 4 {
        print_usage();
        eprintln!(
            "{} incorrect count of arguments: expected 4, found {}",
            "Error: ".red().bold(),
            args.len()
        );

        std::process::exit(1);
    }

    Arguments {
        target: args[0].clone(),
        replacement: args[1].clone(),
        filename: args[2].clone(),
        output: args[3].clone(),
    }
}

fn replace(target: &str, replacement: &str, text: &str) -> Result<String, regex::Error> {
    let regex = Regex::new(target)?;
    Ok(regex.replace_all(text, replacement).to_string())
}

fn main() {
    let args = parse_args();

    let data = match fs::read_to_string(&args.filename) {
        Ok(v) => v,
        Err(e) => {
            eprintln!(
                "{} unable to read content of file '{}': {:?}",
                "Error: ".red().bold(),
                args.filename,
                e
            );
            std::process::exit(1);
        }
    };

    let replaced_data = match replace(&args.target, &args.replacement, &data) {
        Ok(v) => v,
        Err(e) => {
            eprintln!(
                "{} unable to replacer content of file '{}': {:?}",
                "Error: ".red().bold(),
                args.filename,
                e
            );
            std::process::exit(1);
        }
    };

    match fs::write(&args.output, &replaced_data) {
        Ok(_) => {}
        Err(e) => {
            eprintln!(
                "{} unable to write content to file '{}': {:?}",
                "Error: ".red().bold(),
                args.filename,
                e
            );
            std::process::exit(1);
        }
    }
}
