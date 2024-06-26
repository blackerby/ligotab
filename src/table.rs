use crate::format::Format;
use csv::{Error, ReaderBuilder};
use std::{fmt::Display, io::BufRead};

pub struct Table {
    header_delimiter: &'static str,
    rule_char: Option<char>,
    rule_intersection: Option<char>,
    rows: Vec<Vec<String>>,
    row_delimiter: char,
    widths: Option<Vec<usize>>,
}

trait Escape {
    fn escape_brackets(&self) -> String;
    fn escape_pipe(&self) -> String;
}

impl Escape for String {
    fn escape_pipe(&self) -> Self {
        self.replace("|", "\\|")
    }

    fn escape_brackets(&self) -> Self {
        self.replace("{", "\\{").replace("}", "\\}")
    }
}

impl Table {
    pub fn new(rdr: Box<dyn BufRead>, delimiter: u8, format: Format) -> Result<Table, Error> {
        let mut reader = ReaderBuilder::new()
            .delimiter(delimiter)
            .has_headers(false)
            .from_reader(rdr);

        let mut rows: Vec<Vec<String>> = Vec::new();

        for result in reader.records() {
            let record = result?
                .iter()
                .map(|s| s.to_owned().escape_pipe().escape_brackets())
                .collect();
            rows.push(record);
        }

        match format {
            Format::Confluence => {
                return Ok(Table::create(rows, "||", None, None, '|', None));
            }
            ref f => {
                let row_widths: Vec<Vec<usize>> = rows
                    .iter()
                    .map(|row| row.iter().map(|s| s.len()).collect::<Vec<_>>())
                    .collect();

                let col_widths = Some(
                    (0..row_widths[0].len())
                        .map(|i| row_widths.iter().map(|w| w[i]).max().unwrap_or(0))
                        .collect(),
                );

                match f {
                    Format::Org => Ok(Table::create(
                        rows,
                        "|",
                        Some('-'),
                        Some('+'),
                        '|',
                        col_widths,
                    )),
                    Format::Markdown => Ok(Table::create(
                        rows,
                        "|",
                        Some('-'),
                        Some('|'),
                        '|',
                        col_widths,
                    )),
                    _ => unreachable!("Unrecognized output format"),
                }
            }
        }
    }

    fn create(
        rows: Vec<Vec<String>>,
        header_delimiter: &'static str,
        rule_char: Option<char>,
        rule_intersection: Option<char>,
        row_delimiter: char,
        widths: Option<Vec<usize>>,
    ) -> Table {
        Table {
            rows,
            header_delimiter,
            rule_char,
            rule_intersection,
            row_delimiter,
            widths,
        }
    }

    fn format_header(&self) -> String {
        let first_row = &self.rows[0];
        let formatted_row = if let Some(widths) = &self.widths {
            first_row
                .iter()
                .zip(widths)
                .map(|(d, w)| format!("{d:<w$}"))
                .collect::<Vec<String>>()
        } else {
            first_row.to_owned()
        };

        let header = formatted_row.join(&self.header_delimiter);
        format!(
            "{}{}{}\n",
            self.header_delimiter, header, self.header_delimiter,
        )
    }

    fn format_rule(&self) -> String {
        if let Some(widths) = &self.widths {
            let rule: String = widths
                .iter()
                .map(|w| self.rule_char.unwrap().to_string().repeat(*w))
                .collect::<Vec<_>>()
                .join(self.rule_intersection.unwrap().to_string().as_str());

            format!("{}{}{}\n", self.row_delimiter, rule, self.row_delimiter,)
        } else {
            String::new()
        }
    }

    fn format_rows(&self) -> String {
        let rows = &self.rows[1..];
        let data = if let Some(widths) = &self.widths {
            rows.to_owned()
                .iter()
                .map(|row| {
                    row.iter()
                        .zip(widths.iter())
                        .map(|(d, w)| format!("{d:<w$}"))
                        .collect::<Vec<_>>()
                })
                .collect()
        } else {
            rows.to_owned()
        };

        let formatted_rows: String = data
            .iter()
            .map(|row| {
                format!(
                    "{}{}{}",
                    self.row_delimiter,
                    row.join(&self.row_delimiter.to_string()),
                    self.row_delimiter
                )
            })
            .collect::<Vec<String>>()
            .join("\n");

        format!("{}", formatted_rows)
    }

    fn compose(&self) -> String {
        let header = self.format_header();
        let rule = self.format_rule();
        let rows = self.format_rows();

        format!("{}{}{}", header, rule, rows)
    }
}

impl Display for Table {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.compose())
    }
}
