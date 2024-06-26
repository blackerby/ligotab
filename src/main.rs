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
    /// Set the delimiter character. Expand escape characters in the shell, e.g., `$'\t'`.
    #[arg(short, long, default_value = ",")]
    delimiter: char,
    /// (Optional) Set the record terminator character.
    #[arg(short, long)]
    terminator: Option<char>,
    /// (Optional) Set the file commenting character.
    #[arg(short, long)]
    comment_char: Option<char>,
    /// Disable quoting when reading file.
    #[arg(short, long, default_value_t = false)]
    quoting: bool,
    /// Set the quoting character.
    #[arg(short = 'u', long, default_value = "\"")]
    quote_char: char,
    /// Disable interpreting double quote as escape
    #[arg(short = 'b', long, default_value_t = false)]
    double_quote: bool,
    /// Set the output format for the table. Valid formats are `markdown`, `confluence`, and `org`.
    #[arg(short, long, default_value = "markdown")]
    output_format: String,
}

fn main() {
    let cli = Cli::parse();
    let quoting_enabled = !cli.quoting;
    let double_quoting_enabled = !cli.double_quote;

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

    let table = Table::new(
        reader,
        cli.delimiter as u8,
        cli.terminator,
        cli.comment_char.map(|c| c as u8),
        quoting_enabled,
        cli.quote_char as u8,
        double_quoting_enabled,
        Format::from(cli.output_format),
    );

    match table {
        Ok(t) => println!("{t}"),
        Err(e) => eprintln!("{e}"),
    }
}
