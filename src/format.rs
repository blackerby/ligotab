use std::process;

#[derive(PartialEq)]
pub enum Format {
    Markdown,
    Org,
    Confluence,
    ReStructuredText,
}

impl From<String> for Format {
    fn from(value: String) -> Self {
        match value.as_ref() {
            "markdown" | "md" | "m" => Self::Markdown,
            "org" | "orgmode" | "o" => Self::Org,
            "confluence" | "c" => Self::Confluence,
            "restructured-text" | "rst" | "r" => Self::ReStructuredText,
            _ => {
                eprintln!("Unsupported output format: {value}");
                process::exit(3);
            }
        }
    }
}
