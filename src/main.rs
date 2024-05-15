// mod
mod utils;

// std lib
use std::io::{self, BufRead, Write};
use std::path::PathBuf;
use std::process::exit;
use std::fs::File;

// libs
use clap::{arg, command, value_parser};

fn main() {
    utils::banner();
    utils::copyright();

    let matches = command!()
        .arg(
            arg!(
                -d --domain <domain_name> "Sets the domain"
            )
                .required(true)
                .value_parser(value_parser!(String)),
        )
        .arg(
            arg!(
                -i --input <input_file> "Sets the input file"
            )
                .required(true)
                .value_parser(value_parser!(PathBuf)),
        )
        .arg(
            arg!(
                -o --output <output_file> "Sets the output file"
            )
                .required(true)
                .value_parser(value_parser!(PathBuf)),
        )
        .get_matches();

    if let Some(domain) = matches.get_one::<String>("domain") {
        println!("Value of domain : {}", domain);
        if let Some(input_path) = matches.get_one::<PathBuf>("input") {
            if input_path.exists() {
                println!("Value of input file : {}", input_path.display());
                if let Some(output_path) = matches.get_one::<PathBuf>("output") {
                    let file = File::create(output_path).expect("Failed to create file");
                    println!("Output file : {}", output_path.display());

                    subgen(domain, input_path.to_path_buf(), file).expect("Error while calling subgen");
                }
            } else {
                println!("File does not exist");
                exit(0);
            }
        }
    }
}

fn subgen(domain: &str, input_path: PathBuf, mut filex: File) -> Result<(), io::Error> {

    let file = File::open(input_path)?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let subdomain = format!("{}.{}", line.unwrap(), domain);
        filex.write_all(subdomain.as_bytes()).unwrap();
        filex.write_all("\n".as_bytes()).unwrap();
    }
    Ok(())
}