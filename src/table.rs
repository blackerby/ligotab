use crate::escape::Escape;
use crate::format::Format;
use csv::{Error, ReaderBuilder, Terminator};
use std::{fmt::Display, io::BufRead};

#[derive(Debug, PartialEq)]
pub struct Table {
    header_delimiter: &'static str,
    rule_char: Option<char>,
    border_char: Option<char>,
    intersection: Option<char>,
    rows: Vec<Vec<String>>,
    row_delimiter: char,
    widths: Option<Vec<usize>>,
}

impl Table {
    pub fn new(
        rdr: Box<dyn BufRead>,
        delimiter: u8,
        terminator: Option<char>,
        comment_char: Option<u8>,
        quoting: bool,
        quote_char: u8,
        double_quote: bool,
        format: Format,
    ) -> Result<Table, Error> {
        let mut binding = ReaderBuilder::new();
        let mut rows: Vec<Vec<String>> = Vec::new();

        let mut reader_builder = binding
            .delimiter(delimiter)
            .has_headers(false)
            .comment(comment_char)
            .quote(quote_char)
            .double_quote(double_quote)
            .quoting(quoting);

        if let Some(terminator) = terminator {
            reader_builder = reader_builder.terminator(Terminator::Any(terminator as u8));
        }

        let mut reader = reader_builder.from_reader(rdr);

        for result in reader.records() {
            let record = result?
                .iter()
                .map(|s| s.to_owned().escape_pipe().escape_brackets())
                .collect();
            rows.push(record);
        }

        match format {
            Format::Confluence => {
                return Ok(Table::create(rows, "||", None, None, None, '|', None));
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
                    Format::ReStructuredText => Ok(Table::create(
                        rows,
                        "|",
                        Some('='),
                        Some('-'),
                        Some('+'),
                        '|',
                        col_widths,
                    )),
                    Format::Org => Ok(Table::create(
                        rows,
                        "|",
                        Some('-'),
                        None,
                        Some('+'),
                        '|',
                        col_widths,
                    )),
                    Format::Markdown => Ok(Table::create(
                        rows,
                        "|",
                        Some('-'),
                        None,
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
        border_char: Option<char>,
        intersection: Option<char>,
        row_delimiter: char,
        widths: Option<Vec<usize>>,
    ) -> Table {
        Table {
            rows,
            header_delimiter,
            rule_char,
            border_char,
            intersection,
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
                .join(self.intersection.unwrap().to_string().as_str());

            format!("{}{}{}\n", self.row_delimiter, rule, self.row_delimiter,)
        } else {
            String::new()
        }
    }

    fn format_border(&self) -> Option<String> {
        if let Some(border_char) = &self.border_char {
            let widths = self.widths.as_ref().unwrap();
            let intersection = self.intersection.unwrap();

            let border: String = widths
                .iter()
                .map(|w| border_char.to_string().repeat(*w))
                .collect::<Vec<_>>()
                .join(intersection.to_string().as_str());

            Some(format!("{}{}{}\n", intersection, border, intersection))
        } else {
            None
        }
    }

    fn format_rows(&self) -> Vec<String> {
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

        data.iter()
            .map(|row| {
                format!(
                    "{}{}{}",
                    self.row_delimiter,
                    row.join(&self.row_delimiter.to_string()),
                    self.row_delimiter
                )
            })
            .collect::<Vec<String>>()
    }

    fn compose(&self) -> String {
        let header = self.format_header();
        let rule = self.format_rule();
        let rows = self.format_rows();

        if let Some(border) = self.format_border() {
            let mut row_string = String::new();

            for row in rows {
                row_string.push_str(row.as_str());
                row_string.push('\n');
                row_string.push_str(border.as_str());
            }

            format!("{}{}{}{}", border, header, rule, row_string.trim(),)
        } else {
            let row_string = rows.join("\n");
            format!("{}{}{}", header, rule, row_string)
        }
    }
}

impl Display for Table {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.compose())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs::File, io::BufReader};

    #[test]
    fn test_new_csv_markdown() {
        let file = File::open("tests/data/customers-1.csv").unwrap();
        let reader = Box::new(BufReader::new(file));
        let rows = vec![
            vec![
                "Index",
                "Customer Id",
                "First Name",
                "Last Name",
                "Company",
                "City",
                "Country",
                "Phone 1",
                "Phone 2",
                "Email",
                "Subscription Date",
                "Website",
            ]
            .iter()
            .map(|s| s.to_string())
            .collect(),
            vec![
                "1",
                "DD37Cf93aecA6Dc",
                "Sheryl",
                "Baxter",
                "Rasmussen Group",
                "East Leonard",
                "Chile",
                "229.077.5154",
                "397.884.0519x718",
                "zunigavanessa@smith.info",
                "2020-08-24",
                "http://www.stephenson.com/",
            ]
            .iter()
            .map(|s| s.to_string())
            .collect(),
        ];

        let widths = Some(vec![5, 15, 10, 9, 15, 12, 7, 12, 16, 24, 17, 26]);

        let got = Table::new(reader, b',', None, None, true, b'"', true, Format::Markdown).unwrap();

        let want = Table {
            header_delimiter: "|",
            rule_char: Some('-'),
            border_char: None,
            intersection: Some('|'),
            rows,
            row_delimiter: '|',
            widths,
        };

        assert_eq!(got, want);
    }

    #[test]
    fn test_new_csv_markdown_comment() {
        let file = File::open("tests/data/customers-1-comment.csv").unwrap();
        let reader = Box::new(BufReader::new(file));
        let rows = vec![vec![
            "Index",
            "Customer Id",
            "First Name",
            "Last Name",
            "Company",
            "City",
            "Country",
            "Phone 1",
            "Phone 2",
            "Email",
            "Subscription Date",
            "Website",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect()];

        let widths = Some(vec![5, 11, 10, 9, 7, 4, 7, 7, 7, 5, 17, 7]);

        let got = Table::new(
            reader,
            b',',
            None,
            Some(b'#'),
            true,
            b'"',
            true,
            Format::Markdown,
        )
        .unwrap();

        let want = Table {
            header_delimiter: "|",
            rule_char: Some('-'),
            border_char: None,
            intersection: Some('|'),
            rows,
            row_delimiter: '|',
            widths,
        };

        assert_eq!(got, want);
    }
}
