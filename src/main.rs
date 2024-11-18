use clap::{Arg, Command};
use csv::ReaderBuilder;
use std::error::Error;
use std::fs::File;
use std::process;

fn parse_csv(file_path: &str) -> Result<(), Box<dyn Error>> {
    let file = File::open(file_path)?;
    let mut rdr = ReaderBuilder::new().has_headers(true).from_reader(file);

    for result in rdr.records() {
        match result {
            Ok(record) => println!("{:?}", record),
            Err(err) => eprintln!("Error reading record: {}", err),
        }
    }
    Ok(())
}

fn show_credits() {
    println!("CSV Parser CLI\nCreated by: Moshkovskyi Ivan\nVersion: 1.0");
}

fn main() {
    let matches = Command::new("csv-parser")
        .version("1.0")
        .author("Moshkovskyi Ivan <i.moshkovskyi@ukma.edu.ua>")
        .about("CLI tool to parse CSV files with custom help messages")
        .help_template(
            "\
{bin} {version}
{about}

USAGE:
    {usage}

COMMANDS:
{all-args}

ADDITIONAL INFORMATION:
    For detailed help on a command, run '{bin} <command> --help'.

CREDITS:
    Created by: Moshkovskyi Ivan
    Version: 1.0
            ")
        .subcommand(
            Command::new("parse")
                .about("Parse a CSV file")
                .arg(
                    Arg::new("file")
                        .required(true)
                        .help("The path to the CSV file to be parsed"),
                )
                .help_template(
                    "\
COMMAND: parse
{about}

USAGE:
    csv-parser parse <file_path>

OPTIONS:
{all-args}
                    ",
                ),
        )
        .subcommand(
            Command::new("credits")
                .about("Show credits information")
                .help_template(
                    "\
COMMAND: credits
{about}

USAGE:
    csv-parser credits

DESCRIPTION:
    Displays the credits and version of the CSV Parser CLI tool.
                    ",
                ),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("parse", sub_m)) => {
            let file_path = sub_m.get_one::<String>("file").expect("file is required");
            if let Err(e) = parse_csv(file_path) {
                eprintln!("Error parsing CSV: {}", e);
                process::exit(1);
            }
        }
        Some(("credits", _)) => show_credits(),
        _ => {
            println!("\nCustom Help:\n");
            println!("Use 'csv-parser --help' to see available commands.");
        }
    }
}