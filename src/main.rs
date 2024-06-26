use clap::Parser;
use ligotab::format::Format;
use ligotab::table::Table;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::process;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Path to delimiter-separated value file.
    #[arg(default_value = "-")]
    path: String,
    /// Delimiter character. Expand escape characters in the shell, e.g., `$'\t'`.
    #[arg(short, long, default_value = ",")]
    delimiter: char,
    /// Output format for the table. Valid formats are `markdown`, `confluence`, and `org`.
    #[arg(short, long, default_value = "markdown")]
    output_format: String,
}

fn main() {
    let cli = Cli::parse();

    let reader: Box<dyn BufRead> = match cli.path.as_str() {
        "-" => Box::new(BufReader::new(io::stdin())),
        path => match File::open(path) {
            Ok(f) => Box::new(BufReader::new(f)),
            Err(e) => {
                eprintln!("{e}");
                process::exit(1);
            }
        },
    };

    let table = Table::new(reader, cli.delimiter as u8, Format::from(cli.output_format));

    match table {
        Ok(t) => println!("{t}"),
        Err(e) => eprintln!("{e}"),
    }
}
